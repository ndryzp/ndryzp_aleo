// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use leo_ast::*;
use leo_errors::TypeCheckerError;

use crate::{Declaration, TypeChecker, VariableSymbol};

use super::director::Director;

impl<'a> ProgramVisitor<'a> for TypeChecker<'a> {}

impl<'a> ProgramVisitorDirector<'a> for Director<'a> {
    fn visit_function(&mut self, input: &'a Function) {
        if let VisitResult::VisitChildren = self.visitor_ref().visit_function(input) {
            self.visitor.has_return = false;
            self.visitor.symbol_table.clear_variables();
            self.visitor.parent = Some(input.name());
            input.input.iter().for_each(|i| {
                let input_var = i.get_variable();
                self.visitor.validate_ident_type(&Some(input_var.type_));

                if let Err(err) = self.visitor.symbol_table.insert_variable(
                    input_var.identifier.name,
                    VariableSymbol {
                        type_: &input_var.type_,
                        span: input_var.identifier.span(),
                        declaration: Declaration::Input(input_var.mode()),
                    },
                ) {
                    self.visitor.handler.emit_err(err);
                }
            });
            self.visit_block(&input.block);

            if !self.visitor.has_return {
                self.visitor
                    .handler
                    .emit_err(TypeCheckerError::function_has_no_return(input.name(), input.span()).into());
            }
        }
    }

    fn visit_circuit(&mut self, input: &'a Circuit) {
        // circuit Foo { x: u8, y: u8 };
        todo!()
        // if let VisitResult::VisitChildren = self.visitor_ref().visit_function(input) {
        //     self.
        // }
    }
}
