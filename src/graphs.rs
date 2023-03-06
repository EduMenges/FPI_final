use ordered_float::OrderedFloat;
use petgraph::prelude::UnGraph;

use crate::{helpers::{CoordinatesF, Connected}, segmentation::{GeoSegment, ImageSegments}};

pub trait EuclideanDistance {
    fn calc_euclidean_distance(&self, other: &Self) -> f64 {
        let centr_s = self.calc_centroid();
        let centr_o = other.calc_centroid();

        let x_dist = (centr_s.0 - centr_o.0).abs();
        let y_dist = (centr_s.1 - centr_o.1).abs();

        (x_dist.powi(2) + y_dist.powi(2)).sqrt()
    }

    fn calc_centroid(&self) -> CoordinatesF;
}

impl EuclideanDistance for GeoSegment {
    fn calc_centroid(&self) -> CoordinatesF {
        self.centroid
    }
}

pub type SegmentGraph = UnGraph<GeoSegment, ()>;

pub fn mount_graph(f_segments: ImageSegments, b_segments: ImageSegments) -> SegmentGraph {
    let mut res = SegmentGraph::new_undirected();

    for segment in f_segments.into_iter() {
        res.add_node(segment);
    }

    connect_boundaries(&mut res);

    let mut b_segments = b_segments.into_iter();
    let division = res.add_node(b_segments.next().unwrap());

    for segment in b_segments {
        res.add_node(segment);
    }

    connect_neighbours(&mut res, division.index());

    res
}

fn connect_boundaries(seg_graph: &mut SegmentGraph) {
    let mut nodes = seg_graph.node_indices();

    while let Some(node) = nodes.next() {
        let remaning_nodes = nodes.clone();

        for other_node in remaning_nodes {
            if seg_graph[node].seg.is_connected(&seg_graph[other_node].seg) {
                seg_graph.add_edge(node, other_node, ());
            }
        }
    }
}

const K_VALUE: usize = 6;
fn connect_neighbours(seg_graph: &mut SegmentGraph, division: usize) {
    let background_nodes = seg_graph.node_indices().skip(division);
    let total_background_nodes = seg_graph.node_count() - division;

    for f_node in seg_graph.node_indices().take(division) {
        let mut min_tree = Vec::with_capacity(total_background_nodes);

        for b_node in background_nodes.clone() {
            min_tree.push((OrderedFloat(seg_graph[b_node].calc_euclidean_distance(&seg_graph[f_node])), b_node));
        }

        min_tree.sort_by_key(|(distance, _)| *distance);

        for (_, b_node) in min_tree.into_iter().take(K_VALUE) {
            seg_graph.add_edge(f_node, b_node, ());
        }
    }
}

#[cfg(test)]
mod tests {
    use image::io::Reader;

    use crate::segmentation::ImgSegmentation;

    use super::mount_graph;

    #[test]
    fn test_graph() {
        let img = Reader::open(r"img_segments\segments.tif")
            .unwrap()
            .decode()
            .unwrap()
            .to_luma_alpha8();

        let segments = ImgSegmentation::segment_img(&img);
        let b_segments = ImgSegmentation::segment_img(&img);

        let _graph = mount_graph(segments, b_segments);
    }
}
