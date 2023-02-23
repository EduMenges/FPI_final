use petgraph::{
    prelude::UnGraph,
    visit::{IntoNodeReferences, NodeFilteredNodes},
};

use crate::segmentation::{ImageSegment, ImageSegments};

pub type SegmentGraph = UnGraph<ImageSegment, ()>;

pub fn mount_graph(f_segments: ImageSegments, b_segments: ImageSegments) -> SegmentGraph {
    let mut res = SegmentGraph::new_undirected();

    for segment in f_segments.into_iter() {
        res.add_node(segment);
    }

    res
}

fn connect_boundaries(segment: &mut SegmentGraph) {
    let mut remaining_indices = segment.node_indices();

    while let Some(test_node) = remaining_indices.next() {
        let iterating_indices = remaining_indices.clone();

        for other_node in iterating_indices {}
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
