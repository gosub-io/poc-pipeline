# Render pipeline POC

This repo contains a proof of concept for a render pipeline that can be used to render a webpage.

The pipeline consists of multiple stages. Each stage is responsible for transforming the data from the previous stage into a new format.
Depending on any changes in the data, the pipeline can be re-run to update the rendering. It's possible that not the whole pipeline needs
to be re-run, but only a subset of the stages.

The stages:

 - Rendertree generation
 - Layout tree generation
 - Layering
 - Tiling
 - Rendering
 - Compositing

The first step is generating the render tree. It will convert a DOM tree together with CSS styles into a tree of nodes that are needed for 
generating layout. The node IDs in the render-tree are the same as the node IDs in the DOM tree.

The second step is to generate a layout. For this we use Taffy to compute all the layout elements. The layout elements are the building blocks
for the layout tree.

The third step is to generate layers. Layers are used to optimize rendering. They are used to group elements that can be rendered together.
For now, images are stored on their own layer. All the other elements are stored in the same layer (layer 0)

[ not yet implemented ]
The next step is tiling. Here we convert the layout tree into elements of 256x256 pixels (tiles). This is done to optimize rendering. Only the
tiles that are visible on the screen are rendered.

[ not yet implemented ]
The next step is actual painting of tiles. This is done by rendering the tiles that are visible on the screen.

[ not yet implemented ]
The final step is compositing. Here we combine the visible tiles in the layers onto the screen. When we have CSS animations like transitions, we
do not need to repaint the tiles, but merely update the position of the tiles (or their opacity). The compositing will take care of this and returns 
fully rendered frame.


## Passing of data
Each stage will take the data from the previous stage and transform it into a new format. Note that the data from earlier stages are still available 
by wrapping these structures.

For instance, the layering stage will take the layout tree and the render tree as input. The output of the layering stage is a list of layers.
Note that we have a wrapped layouttree, which in turn has a wrapped render tree which in turn has a wrapped DOM document.

```
LayerList
    - wrapped[layout_tree]
        - wrapped[render_tree]: RenderTree
            - wrapped[doc]: Document
                - root: Node
                    - node_id: NodeId
                    - children: Vec<Node>
                    - node_type: NodeType
            - root: RenderNode
                - node_id: NodeId
                - children: Vec<RenderNode>
        - taffy_tree
        - taffy_root_id
        - root_layout_element: LayoutElementNode
            - node_id: LayoutElementId
            - dom_node_id: DomNodeId
            - taffy_node_id: TaffyNodeId
            - children: Vec<LayoutElementNode>
            - box_model: BoxModel
        - node_mapping
    - layers: Vec<Layer>
            - id: LayerId
            - order: isize
            - elements: Vec<NodeId>

```