"""
Aytham Semantic Graph Validator v0.1

Minimal research prototype.
This is not a compiler or language runtime.
It validates semantic graph requirements and produces explanations.
"""


class SemanticGraphValidator:
    def __init__(self, graph):
        self.graph = graph
        self.claims = {
            (c["subject"], c["property"]): c
            for c in graph.get("claims", [])
        }

    def validate_action(self, action_id):
        action = next(
            (a for a in self.graph.get("actions", []) if a["id"] == action_id),
            None,
        )

        if action is None:
            return {
                "valid": False,
                "reason": f"Unknown action: {action_id}",
            }

        missing = []

        for requirement in action.get("requires", []):
            key = (requirement["subject"], requirement["property"])
            claim = self.claims.get(key)

            if claim is None or claim.get("value") != requirement.get("value"):
                missing.append(requirement)

        if missing:
            return {
                "valid": False,
                "explanation": {
                    "message": f"Cannot execute {action_id}",
                    "missing_claims": missing,
                },
            }

        return {
            "valid": True,
            "explanation": {
                "message": f"Action {action_id} can execute",
                "requirements_satisfied": action.get("requires", []),
            },
        }
