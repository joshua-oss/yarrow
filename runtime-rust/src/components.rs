// INTERNAL IMPORTS
extern crate yarrow_validator;
use yarrow_validator::yarrow;

use crate::base::{
    NodeArguments, NodeEvaluation, FieldEvaluation,
    parse_proto_array, get_f64, get_array_f64
};
use crate::statistics;

// STANDARD LIB IMPORTS
use std::str::FromStr;
use std::collections::HashMap;

// EXTERNAL IMPORTS
use ndarray::prelude::*;

extern crate csv;
extern crate num;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

impl Evaluable for yarrow::Literal {
    fn evaluate(&self, arguments: &NodeArguments) -> NodeEvaluation {
        hashmap!["data".to_owned() => parse_proto_array(self.value.unwrap())]
    }
}

// TODO: replace DataSource with Index, add Table component to unify trait api
impl Evaluable for yarrow::DataSource {
    fn evaluate(&self, arguments: &NodeArguments) -> NodeEvaluation {

        let table = dataset.tables.get(&datasource.dataset_id).unwrap();
        let data = match table.value.as_ref().unwrap() {
            yarrow::table::Value::FilePath(path) => {

                fn get_column<T>(path: &String, column: &String) -> Vec<T>
                    where T: FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug {
                    let mut rdr = csv::Reader::from_path(path).unwrap();
                    rdr.deserialize().map(|result| {
                        let record: HashMap<String, String> = result.unwrap();
//                    println!("{:?}", record);
                        record[column].parse::<T>().unwrap()
                    }).collect()
                }

                match arguments.get("datatype").unwrap() {
                    FieldEvaluation::Str(x) => Ok(match x.first().unwrap().as_ref() {
                        "BYTES" =>
                            Ok(FieldEvaluation::Bytes(Array1::from(get_column::<u8>(&path, &datasource.column_id)).into_dyn())),
                        "BOOL" =>
                            Ok(FieldEvaluation::Bool(Array1::from(get_column::<bool>(&path, &datasource.column_id)).into_dyn())),
                        "I64" =>
                            Ok(FieldEvaluation::I64(Array1::from(get_column::<i64>(&path, &datasource.column_id)).into_dyn())),
                        "F64" =>
                            Ok(FieldEvaluation::F64(Array1::from(get_column::<f64>(&path, &datasource.column_id)).into_dyn())),
                        "STRING" =>
                            Ok(FieldEvaluation::Str(Array1::from(get_column::<String>(&path, &datasource.column_id)).into_dyn())),
                        _ => Err("Datatype is not recognized.")
                    }.unwrap()),
                    _ => Err("Datatype must be a string.")
                }
            },
            yarrow::table::Value::Literal(value) => Ok(parse_proto_array(&value)),
            _ => Err("Only file paths are supported")
        }.unwrap();

        hashmap!["data".to_owned() => data]
    }
}

impl Evaluable for yarrow::Add {
    fn evaluate(&self, arguments: &NodeArguments) -> NodeEvaluation {
        match (arguments.get("left").unwrap(), arguments.get("right").unwrap()) {
            (FieldEvaluation::F64(x), FieldEvaluation::F64(y)) =>
                Ok(hashmap!["data".to_string() => FieldEvaluation::F64(x + y)]),
            (FieldEvaluation::I64(x), FieldEvaluation::I64(y)) =>
                Ok(hashmap!["data".to_string() => FieldEvaluation::I64(x + y)]),
            _ => Err("Add: Either the argument types are mismatched or non-numeric.")
        }.unwrap()
    }
}

impl Evaluable for yarrow::Subtract {
    fn evaluate(&self, arguments: &NodeArguments) -> NodeEvaluation {
        match (arguments.get("left").unwrap(), arguments.get("right").unwrap()) {
            (FieldEvaluation::F64(x), FieldEvaluation::F64(y)) =>
                Ok(hashmap!["data".to_string() => FieldEvaluation::F64(x - y)]),
            (FieldEvaluation::I64(x), FieldEvaluation::I64(y)) =>
                Ok(hashmap!["data".to_string() => FieldEvaluation::I64(x - y)]),
            _ => Err("Subtract: Either the argument types are mismatched or non-nnumeric.")
        }.unwrap()
    }
}

impl Evaluable for yarrow::Multiply {
    fn evaluate(&self, arguments: &NodeArguments) -> NodeEvaluation {
        match (arguments.get("left").unwrap(), arguments.get("right").unwrap()) {
            (FieldEvaluation::F64(x), FieldEvaluation::F64(y)) =>
                Ok(hashmap!["data".to_string() => FieldEvaluation::F64(x * y)]),
            (FieldEvaluation::I64(x), FieldEvaluation::I64(y)) =>
                Ok(hashmap!["data".to_string() => FieldEvaluation::I64(x * y)]),
            _ => Err("Divide: Either the argument types are mismatched or non-numeric.")
        }.unwrap()
    }
}

impl Evaluable for yarrow::Divide {
    fn evaluate(&self, arguments: &NodeArguments) -> NodeEvaluation {
        match (arguments.get("left").unwrap(), arguments.get("right").unwrap()) {
            (FieldEvaluation::F64(x), FieldEvaluation::F64(y)) =>
                Ok(hashmap!["data".to_string() => FieldEvaluation::F64(x / y)]),
            (FieldEvaluation::I64(x), FieldEvaluation::I64(y)) =>
                Ok(hashmap!["data".to_string() => FieldEvaluation::I64(x / y)]),
            _ => Err("Multiply: Either the argument types are mismatched or non-numeric.")
        }.unwrap()
    }
}

impl Evaluable for yarrow::Power {
    fn evaluate(&self, arguments: &NodeArguments) -> NodeEvaluation {
        let power: f64 = get_f64(&arguments, "right");
        let data = get_array_f64(&arguments, "left");
        hashmap!["data".to_string() => FieldEvaluation::F64(data.mapv(|x| x.powf(power)))]
    }
}

impl Evaluable for yarrow::Negate {
    fn evaluate(&self, arguments: &NodeArguments) -> NodeEvaluation {
        match arguments.get("data").unwrap() {
            FieldEvaluation::F64(x) =>
                Ok(hashmap!["data".to_string() => FieldEvaluation::F64(-x)]),
            FieldEvaluation::I64(x) =>
                Ok(hashmap!["data".to_string() => FieldEvaluation::I64(-x)]),
            _ => Err("Negate: Argument must be numeric.")
        }.unwrap()
    }
}


impl Evaluable for yarrow::DpMean {
    fn evaluate(&self, arguments: &NodeArguments) -> NodeEvaluation {
        let data: FieldEvaluation = match yarrow::Mechanism::from_i32(self.mechanism).unwrap() {
            yarrow::Mechanism::Laplace => Ok(FieldEvaluation::F64(Array::from_elem((), statistics::mean::laplace(
                self.epsilon,
                get_f64(&arguments, "num_records"),
                get_array_f64(&arguments, "data"),
                get_f64(&arguments, "minimum"),
                get_f64(&arguments, "maximum")
            )).into_dyn())),
            _ => Err("Mean: Unknown algorithm type.")
        }.unwrap();
        // println!("dpmean");
        hashmap!["data".to_string() => data]
    }
}

impl Evaluable for yarrow::DpVariance {
    fn evaluate(&self, arguments: &NodeArguments) -> NodeEvaluation {
        let data: FieldEvaluation = match yarrow::Mechanism::from_i32(self.mechanism).unwrap() {
            yarrow::Mechanism::Laplace => Ok(FieldEvaluation::F64(Array::from_elem((), statistics::variance::laplace(
                self.epsilon,
                get_f64(&arguments, "num_records"),
                get_array_f64(&arguments, "data"),
                get_f64(&arguments, "minimum"),
                get_f64(&arguments, "maximum")
            )).into_dyn())),
            _ => Err("Variance: Unknown algorithm type.")
        }.unwrap();
        hashmap!["data".to_string() => data]
    }
}

impl Evaluable for yarrow::DpMomentRaw {
    fn evaluate(&self, arguments: &NodeArguments) -> NodeEvaluation {
        let data: FieldEvaluation = match yarrow::Mechanism::from_i32(self.mechanism).unwrap() {
            yarrow::Mechanism::Laplace => Ok(FieldEvaluation::F64(Array::from_elem((), statistics::moment_raw::laplace(
                self.epsilon,
                get_f64(&arguments, "num_records"),
                get_array_f64(&arguments, "data"),
                get_f64(&arguments, "minimum"),
                get_f64(&arguments, "maximum"),
                self.order
            )).into_dyn())),
            _ => Err("Moment Raw: Unknown algorithm type.")
        }.unwrap();
        hashmap!["data".to_string() => data]
    }
}


impl Evaluable for yarrow::DpCovariance {
    fn evaluate(&self, arguments: &NodeArguments) -> NodeEvaluation {
        let data: FieldEvaluation = match yarrow::Mechanism::from_i32(self.mechanism).unwrap() {
            yarrow::Mechanism::Laplace => Ok(FieldEvaluation::F64(Array::from_elem((), statistics::covariance::laplace(
                self.epsilon,
                get_f64(&arguments, "num_records"),
                get_array_f64(&arguments, "data_x"),
                get_array_f64(&arguments, "data_y"),
                get_f64(&arguments, "minimum_x"),
                get_f64(&arguments, "minimum_y"),
                get_f64(&arguments, "maximum_x"),
                get_f64(&arguments, "maximum_y")
            )).into_dyn())),
            _ => Err("Covariance: Unknown algorithm type.")
        }.unwrap();
        hashmap!["data".to_string() => data]
    }
}
