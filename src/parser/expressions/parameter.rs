use crate::{
	api::context::Context,
	bail_err,
	comptime::{memory::VirtualPointer, CompileTime},
	lexer::{Span, TokenType},
	parser::{Parse, TokenQueue, TokenQueueFunctionality},
};

use super::{name::Name, Expression, Spanned, Typed};

#[derive(Debug, Clone)]
pub struct Parameter {
	name: Name,
	parameter_type: Box<Expression>,
	span: Span,
}

impl Parse for Parameter {
	type Output = Parameter;

	fn parse(tokens: &mut TokenQueue, context: &mut Context) -> anyhow::Result<Self::Output> {
		let name = Name::parse(tokens, context)?;
		tokens.pop(TokenType::Colon)?;
		let parameter_type = Expression::parse(tokens, context)?;
		Ok(Parameter {
			span: name.span(context).to(&parameter_type.span(context)),
			name,
			parameter_type: Box::new(parameter_type),
		})
	}
}

impl CompileTime for Parameter {
	type Output = Parameter;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		let evaluated = self.parameter_type.evaluate_at_compile_time(context)?;

		if let Expression::Pointer(_) = &evaluated {
			// nothing to see here...
		} else {
			bail_err! {
				base = "A value that's not fully known at compile-time was used as a parameter type",
				context = context,
			}
		}

		let parameter = Parameter {
			name: self.name.clone(),
			parameter_type: Box::new(evaluated),
			span: self.span,
		};

		context.scope_data.reassign_variable(&self.name, Expression::Parameter(parameter.clone()))?;

		Ok(parameter)
	}
}

impl Spanned for Parameter {
	fn span(&self, _context: &Context) -> Span {
		self.span.to_owned()
	}
}

impl Typed for Parameter {
	fn get_type(&self, context: &mut Context) -> anyhow::Result<VirtualPointer> {
		Ok(self.parameter_type.try_as_literal(context)?.address.unwrap())
	}
}

impl Parameter {
	pub fn name(&self) -> &Name {
		&self.name
	}

	pub fn parameter_type(&self) -> &Expression {
		&self.parameter_type
	}
}
