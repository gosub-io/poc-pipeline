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