from typing import Dict, Optional

from textwrap import dedent, indent


class StatefulMacroRules:
    """Helper to generate macro_rules! code with internal states"""

    def __init__(
        self,
        macro_name: str,
        state_name_pats: Dict[str, str],
        state_initials: Dict[str, str],
        macro_attribute: str = "",
    ):
        self.macro_name = macro_name
        self.private_macro_name = f"_{macro_name}_state"
        self.macro_attribute = macro_attribute
        self.state_name_pats = state_name_pats
        self.state_initials = state_initials
        self.rules_code = []

    def rule(
        self,
        input_pat: str,
        state_appends: Dict[str, str],
        state_replaces: Optional[Dict[str, str]] = None,
        comment: str = "",
    ):
        comment = indent(comment, "// ")
        comment += f"\n// - pattern: {input_pat}"
        input_pat = input_pat.replace("...", "$($_input_rest:tt)*")
        state_fields = {n: f"$($_{n}_rest:tt)*" for n in self.state_name_pats}
        input_state = " ".join(f"({n} [{p}])" for n, p in sorted(state_fields.items()))
        state_fields = {n: f"$($_{n}_rest)*" for n in self.state_name_pats}
        for name, tt in state_appends.items():
            state_fields[name] += f" {tt}"
            comment += f"\n// - state: {name} += {tt}"
        if state_replaces:
            for name, tt in state_replaces.items():
                state_fields[name] = f" {tt}"
                comment += f"\n// - state: {name} = {tt}"
        output_state = " ".join(f"({n} [{p}])" for n, p in sorted(state_fields.items()))
        code = dedent(
            f"""\
            (input [{input_pat}] state [{input_state}]) => {{
                $crate::{self.private_macro_name}!(input [$($_input_rest)*] state [{output_state}]);
            }};"""
        )
        self.rules_code.append(f"{comment.strip()}\n{code}")

    def conclude(self, body: str):
        input_state = " ".join(
            f"({n} [{p}])" for n, p in sorted(self.state_name_pats.items())
        )
        code = dedent(
            f"""\
            // No more input. Conclude the final state.
            (input [] state [{input_state}]) => {{
                {indent(body, ' ' * 16).strip()}
            }};"""
        )
        self.rules_code.append(code)
        all_rules = "\n\n".join(self.rules_code).strip()
        initial_state = self.state_initials.copy()
        for name in self.state_name_pats:
            if name not in initial_state:
                initial_state[name] = ""
        initial_state_str = "".join(
            f"({n} [{v}])" for n, v in sorted(initial_state.items())
        )
        return dedent(
            f"""\
            #[doc(hidden)]
            #[macro_export]
            macro_rules! {self.private_macro_name} {{
                {indent(all_rules, ' ' * 16).strip()}
            }}
            
            {indent(self.macro_attribute, ' ' * 12).strip()}
            #[macro_export]
            macro_rules! {self.macro_name} {{
                {{$($input:tt)*}} => {{
                    $crate::{self.private_macro_name}!(input [$($input)*] state [{initial_state_str}]);
                }};
            }}"""
        )
