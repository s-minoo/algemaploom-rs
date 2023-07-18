#!/usr/bin/env python
import uuid
import json
import graphviz
import argparse


def extend_functions(attr: str, function: dict) -> str:

    func_str: str = ""
    count = 0
    while True:
        if function is None:
            break
        count += 1
        func_str = "{}{}(".format(func_str, function["type"])
        next_func = function.get("inner_function")
        if next_func is None:
            func_str = "{}{}".format(func_str, function["value"])
            break
        function = next_func

    while count > 0:
        func_str += ")"
        count -= 1

    return "{} -> {}".format(attr, func_str)


def parse_operator(operator,
                   parent_id: str | None,
                   dot: graphviz.Graph):

    node_label = operator["type"]
    node_id = str(uuid.uuid4())
    operator_config: dict = operator["config"]

    match operator["type"]:
        case "SerializerOp":
            node_label = "{}\n{}\n Format:{}".format(
                node_label,
                operator_config["template"],
                operator_config["format"])
        case "ExtendOp":
            attr_func_strs = []
            for k, v in operator_config["extend_pairs"].items():
                attr_func_strs.append(extend_functions(k, v))

            node_label = "{}\n{}".format(
                node_label,
                "\n".join(attr_func_strs)
            )
        case "ProjectOp":
            attributes = ",".join(operator_config["projection_attributes"])
            node_label = "{}\n{}".format(node_label, attributes)

            pass
        case "SourceOp":
            node_label = "{}\n file:{}".format(
                node_label, operator_config["path"])
            pass
    dot.node(node_id, node_label)
    if parent_id is not None:
        dot.edge(parent_id, node_id)

    next_op = operator.get("operator")
    if next_op is None:
        return
    parse_operator(next_op, node_id, dot)


def main():
    """TODO: Docstring for function.

    :arg1: TODO
    :returns: TODO

    """

    parser = argparse.ArgumentParser(
        prog="visualizer.py",
        description="Graphviz visualizer for mapping algebra trees")
    parser.add_argument('json_file', type=str,
                        help="JSON file containing the mapping plan")

    parser.add_argument('rml_doc_name', type=str, nargs=1,
                        help="Name of the RML document file from \
                             which the mapping plan was generated")

    args = parser.parse_args()

    file = open(args.json_file)
    mapping_plan = json.load(file)
    dot = graphviz.Graph("mapping_tree", format="png",
                         comment="The mapping plan for {}".format(args.rml_doc_name))

    for tree in mapping_plan:
        parse_operator(tree, parent_id=None,  dot=dot)
    dot.render(directory="output-graphviz")


if __name__ == "__main__":
    main()
