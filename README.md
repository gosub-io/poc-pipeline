# Render pipeline POC

This repo contains a proof of concept for a render pipeline that can be used to render a webpage.

The pipeline consists of multiple stages. Each stage is responsible for transforming the data from the previous stage into a new format or 
updates on current data. Depending on any changes in the data, the pipeline can be re-run to update the rendering. It's possible that not 
the whole pipeline needs to be re-run, but only a subset of the stages.

The stages:

 - Rendertree generation - Convert the DOM tree into a render tree
 - Layout tree generation - Computing the layout of the elements
 - Layering - Grouping elements into layers
 - Tiling - Splitting the layout tree into tiles
 - Painting - Generating paint commands
 - Rasterizing - Executing paint commands onto tiles
 - Compositing - combine the tiles into a final image

The first step is generating the render tree. It will convert a DOM tree together with CSS styles into a tree of nodes that are needed for 
generating layout. The node IDs in the render-tree are the same as the node IDs in the DOM tree.

The second step is to generate a layout. For this we use Taffy to compute all the layout elements. The layout elements are the building blocks
for the layout tree.

The third step is to generate layers. Layers are used to optimize rendering. They are used to group elements that can be rendered together.
If there are elements with some kind of CSS animations, they can be moved to a separate layer, and let the compositor deal with this animation.
This means that we do not need to rerender the layers or tiles, but merely update the position of the layers in the compositor.

The next step is tiling. Here we convert the layout tree into elements of 256x256 pixels (tiles). This is done to optimize rendering dirty elements. 
Only the tiles that are visible on the screen are rendered and cached. When the user scrolls, we only need to render the new tiles that are visible 
on the screen. This however, can be done during idle time in the browser as well. Furthermore, if the user scrolls backwards, older tiles that are
still valid do not have to be rendered again.

The painting generates commands that are needed to render pixels onto the tiles. However, it does not execute this painting. It merely generates
the commands.

The reastering phase will get the tiles and the paint commands and execute the painting per tile into textures.

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