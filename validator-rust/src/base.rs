use super::yarrow;

use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::FromIterator;


pub fn validate_analysis(
    analysis: yarrow::Analysis
) -> yarrow::Validated {

    let mut valid = true;
    let mut errors = Vec::<String>::new();

    // CHECK: analysis graph must be a DAG
//    let mut traversal = VecDeque::from_iter(get_sinks(&analysis).iter());
//    let mut visited_nodes: HashSet<u32> = HashSet::<u32>::new();
//    while !traversal.is_empty() {
//        let visited_node: u32 = traversal.pop_front().unwrap();
//        if visited_nodes.contains(&visited_node) {
//            errors.push("")
//        }
//
//    }

//    Transforms are conformable? (IE df1.a + df2.a when len(df1) != len(df2))
//    Compatible with definition spec
//    Common privacy definition across privatizers

    yarrow::Validated {valid}
}

pub fn compute_privacy(
    analysis: yarrow::Analysis,
    release: yarrow::Release
) -> yarrow::PrivacyUsage {

    yarrow::PrivacyUsage {}
}

pub fn generate_report(
    analysis: yarrow::Analysis,
    release: yarrow::Release
) -> yarrow::Report {

    yarrow::Report {
        value: "{\"key\": \"value\"}".to_owned()
    }
}

pub fn infer_constraints(
    analysis: yarrow::Analysis,
    release: yarrow::Release,
    constraints: yarrow::Constraints
) -> yarrow::Analysis {

    analysis
}

pub fn compute_sensitivities(
    analysis: yarrow::Analysis,
    release: yarrow::Release
) -> yarrow::Sensitivities {

    yarrow::Sensitivities {}
}


pub fn from_accuracy(
    analysis: yarrow::Analysis,
    release: yarrow::Release,
    accuracies: yarrow::Accuracies
) -> yarrow::PrivacyUsageNode {

    yarrow::PrivacyUsageNode {}
}

pub fn to_accuracy(
    analysis: yarrow::Analysis,
    release: yarrow::Release
) -> yarrow::Accuracies {

    yarrow::Accuracies {}
}



pub fn get_release_nodes(analysis: &yarrow::Analysis) -> HashSet<u32> {

    let mut release_node_ids = HashSet::<u32>::new();
    // assume sinks are private
    let sink_node_ids = get_sinks(analysis);
//    println!("sink nodes: {:?}", sink_node_ids);

    // traverse back through arguments until privatizers found
    let mut node_queue = VecDeque::from_iter(sink_node_ids.iter());

    let graph: &HashMap<u32, yarrow::Component> = &analysis.graph;

    while !node_queue.is_empty() {
        let node_id = node_queue.pop_front().unwrap();
        let component = graph.get(&node_id).unwrap();

        if is_privatizer(&component) {
            release_node_ids.insert(*node_id);
        }
        else {
            for field in component.arguments.values() {
                node_queue.push_back(&field.source_node_id);
            }
        }
    }

    return release_node_ids;
}

pub fn get_sinks(analysis: &yarrow::Analysis) -> HashSet<u32> {
    let mut node_ids = HashSet::<u32>::new();
    // start with all nodes
    for node_id in analysis.graph.keys() {
        node_ids.insert(*node_id);
    }

    // remove nodes that are referenced in arguments
    for node in analysis.graph.values() {
        for field in node.arguments.values() {
            node_ids.remove(&field.source_node_id);
        }
    }

    // move to heap, transfer ownership to caller
    return node_ids.to_owned();
}

pub fn is_privatizer(component: &yarrow::Component) -> bool {
    use yarrow::component::Value::*;
    match component.to_owned().value.unwrap() {
        Dpmean(_x) => true,
        _ => false
    }
}


fn is_cyclic() -> bool {

}