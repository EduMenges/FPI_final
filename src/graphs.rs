use petgraph::{
    data::Build,
    prelude::UnGraph,
    visit::{IntoNodeReferences, NodeFilteredNodes},
};

use crate::{
    helpers::SpatialCoordinates,
    segmentation::{Connected, ImageSegment, ImageSegments},
};

pub trait EuclideanDistance {
    fn calc_euclidean_distance(&self, other: &Self) -> f64 {
        let centr_s = self.calc_centroid();
        let centr_o = other.calc_centroid();

        let x_dist = (centr_s.0 - centr_o.0).abs() as f64;
        let y_dist = (centr_s.1 - centr_o.1).abs() as f64;

        (x_dist.powi(2) + y_dist.powi(2)).sqrt()
    }

    fn calc_centroid(&self) -> SpatialCoordinates;
}

impl EuclideanDistance for GeoSegment {
    fn calc_centroid(&self) -> SpatialCoordinates {
        self.centroid
    }
}

impl EuclideanDistance for ImageSegment {
    fn calc_centroid(&self) -> SpatialCoordinates {}
}

pub type SegmentGraph = UnGraph<GeoSegment, ()>;

pub fn mount_graph(f_segments: ImageSegments, b_segments: ImageSegments) -> SegmentGraph {
    let mut res = SegmentGraph::new_undirected();

    for segment in f_segments.into_iter().chain(b_segments.into_iter()) {
        res.add_node(segment);
    }

    connect_boundaries(&mut res);

    res
}

fn connect_boundaries(seg_graph: &mut SegmentGraph) {
    let mut nodes = seg_graph.node_indices();

    while let Some(node) = nodes.next() {
        let remaning_nodes = nodes.clone();

        for other_node in remaning_nodes {
            if seg_graph[node].is_connected(&seg_graph[other_node]) {
                seg_graph.add_edge(node, other_node, ());
            }
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

        let graph = mount_graph(segments, b_segments);
    }
}
