use crate::ast::*;
use crate::error::TogError;
use crate::lexer::{Token, Keyword};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }
    
    pub fn parse(tokens: Vec<Token>) -> Result<Program, TogError> {
        let mut parser = Self::new(tokens);
        let mut statements = Vec::new();
        
        while !parser.is_at_end() {
            statements.push(parser.declaration()?);
        }
        
        Ok(Program { statements })
    }
    
    fn declaration(&mut self) -> Result<Stmt, TogError> {
        if self.match_token(&[Token::Keyword(Keyword::Let)]) {
            self.variable_declaration()
        } else if self.match_token(&[Token::Keyword(Keyword::Struct)]) {
            self.struct_declaration()
        } else if self.match_token(&[Token::Keyword(Keyword::Enum)]) {
            self.enum_declaration()
        } else if self.match_token(&[Token::Keyword(Keyword::Trait)]) {
            self.trait_declaration()
        } else if self.match_token(&[Token::Keyword(Keyword::Impl)]) {
            self.impl_block()
        } else if self.match_token(&[Token::Keyword(Keyword::Fn)]) {
            self.function_declaration()
        } else {
            self.statement()
        }
    }

    fn struct_declaration(&mut self) -> Result<Stmt, TogError> {
        let name = self.consume_identifier()?;
        self.consume(&Token::LeftBrace, "Expected '{' after struct name")?;
        let mut fields = Vec::new();
        let mut methods = Vec::new();

        // Parse fields
        while !self.check(&Token::RightBrace) && !self.check(&Token::Keyword(Keyword::Fn)) && !self.is_at_end() {
            let field_name = self.consume_identifier()?;
            self.consume(&Token::Colon, "Expected ':' after field name")?;
            let field_type = self.parse_type()?;
            fields.push((field_name, Some(field_type)));

            // If there's no comma, it must be the end of fields (or start of methods/end of struct)
            if !self.match_token(&[Token::Comma]) {
                break;
            }
        }

        // Now, parse methods
        while self.match_token(&[Token::Keyword(Keyword::Fn)]) {
            let method_name = self.consume_identifier()?;
            self.consume(&Token::LeftParen, "Expected '(' after method name")?;
            let mut params = Vec::new();
            if !self.check(&Token::RightParen) {
                loop {
                    let param_name = self.consume_identifier()?;
                    let param_type = if self.match_token(&[Token::Colon]) {
                        Some(self.parse_type()?)
                    } else {
                        None
                    };
                    params.push(Param {
                        name: param_name,
                        type_annotation: param_type,
                    });
                    
                    if !self.match_token(&[Token::Comma]) {
                        break;
                    }
                }
            }
            self.consume(&Token::RightParen, "Expected ')' after parameters")?;
            
            let return_type = if self.match_token(&[Token::Arrow]) {
                Some(self.parse_type()?)
            } else {
                None
            };
            
            self.consume(&Token::LeftBrace, "Expected '{' after method signature")?;
            let body = self.block_with_brace_consumed()?;
            
            methods.push(MethodDecl {
                name: method_name,
                params,
                return_type,
                body,
            });
        }

        self.consume(&Token::RightBrace, "Expected '}' after struct body")?;

        Ok(Stmt::StructDef {
            name,
            fields,
            methods,
        })
    }

    fn enum_declaration(&mut self) -> Result<Stmt, TogError> {
        let name = self.consume_identifier()?;
        self.consume(&Token::LeftBrace, "Expected '{' after enum name")?;
        let mut variants = Vec::new();

        // Parse enum variants
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            let variant_name = self.consume_identifier()?;
            
            // Check for associated data type
            let data_type = if self.match_token(&[Token::LeftParen]) {
                let ty = self.parse_type()?;
                self.consume(&Token::RightParen, "Expected ')' after enum variant type")?;
                Some(ty)
            } else {
                None
            };
            
            variants.push(EnumVariant {
                name: variant_name,
                data_type,
            });
            
            // Comma is optional for the last variant
            if !self.match_token(&[Token::Comma]) {
                break;
            }
        }

        self.consume(&Token::RightBrace, "Expected '}' after enum body")?;

        Ok(Stmt::EnumDef { name, variants })
    }

    fn trait_declaration(&mut self) -> Result<Stmt, TogError> {
        let name = self.consume_identifier()?;
        self.consume(&Token::LeftBrace, "Expected '{' after trait name")?;
        let mut methods = Vec::new();

        // Parse trait method signatures
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            self.consume(&Token::Keyword(Keyword::Fn), "Expected 'fn' in trait method")?;
            let method_name = self.consume_identifier()?;
            self.consume(&Token::LeftParen, "Expected '(' after method name")?;
            
            let mut params = Vec::new();
            if !self.check(&Token::RightParen) {
                loop {
                    let param_name = self.consume_identifier()?;
                    let param_type = if self.match_token(&[Token::Colon]) {
                        Some(self.parse_type()?)
                    } else {
                        None
                    };
                    params.push(Param {
                        name: param_name,
                        type_annotation: param_type,
                    });
                    
                    if !self.match_token(&[Token::Comma]) {
                        break;
                    }
                }
            }
            self.consume(&Token::RightParen, "Expected ')' after parameters")?;
            
            let return_type = if self.match_token(&[Token::Arrow]) {
                Some(self.parse_type()?)
            } else {
                None
            };
            
            // Trait methods don't have bodies, just signatures
            // Optionally consume semicolon
            self.match_token(&[Token::Semicolon]);
            
            methods.push(TraitMethod {
                name: method_name,
                params,
                return_type,
            });
        }

        self.consume(&Token::RightBrace, "Expected '}' after trait body")?;

        Ok(Stmt::TraitDef { name, methods })
    }

    fn impl_block(&mut self) -> Result<Stmt, TogError> {
        // impl TraitName for TypeName { ... }
        // or
        // impl TypeName { ... } (inherent impl)
        
        let first_name = self.consume_identifier()?;
        
        let (trait_name, type_name) = if self.match_token(&[Token::Keyword(Keyword::For)]) {
            // impl TraitName for TypeName
            let type_name = self.consume_identifier()?;
            (Some(first_name), type_name)
        } else {
            // impl TypeName (inherent impl)
            (None, first_name)
        };
        
        self.consume(&Token::LeftBrace, "Expected '{' after impl declaration")?;
        let mut methods = Vec::new();

        // Parse method implementations
        while self.match_token(&[Token::Keyword(Keyword::Fn)]) {
            let method_name = self.consume_identifier()?;
            self.consume(&Token::LeftParen, "Expected '(' after method name")?;
            let mut params = Vec::new();
            if !self.check(&Token::RightParen) {
                loop {
                    let param_name = self.consume_identifier()?;
                    let param_type = if self.match_token(&[Token::Colon]) {
                        Some(self.parse_type()?)
                    } else {
                        None
                    };
                    params.push(Param {
                        name: param_name,
                        type_annotation: param_type,
                    });
                    
                    if !self.match_token(&[Token::Comma]) {
                        break;
                    }
                }
            }
            self.consume(&Token::RightParen, "Expected ')' after parameters")?;
            
            let return_type = if self.match_token(&[Token::Arrow]) {
                Some(self.parse_type()?)
            } else {
                None
            };
            
            self.consume(&Token::LeftBrace, "Expected '{' after method signature")?;
            let body = self.block_with_brace_consumed()?;
            
            methods.push(MethodDecl {
                name: method_name,
                params,
                return_type,
                body,
            });
        }

        self.consume(&Token::RightBrace, "Expected '}' after impl body")?;

        Ok(Stmt::ImplBlock {
            trait_name,
            type_name,
            methods,
        })
    }
    
    fn variable_declaration(&mut self) -> Result<Stmt, TogError> {
        let name = self.consume_identifier()?;
        
        let type_annotation = if self.match_token(&[Token::Colon]) {
            Some(self.parse_type()?)
        } else {
            None
        };
        
        self.consume(&Token::Eq, "Expected '=' after variable name")?;
        let value = self.expression()?;
        
        Ok(Stmt::Let {
            name,
            type_annotation,
            value,
        })
    }
    
    fn function_declaration(&mut self) -> Result<Stmt, TogError> {
        let name = self.consume_identifier()?;
        self.consume(&Token::LeftParen, "Expected '(' after function name")?;
        
        let mut params = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                let param_name = self.consume_identifier()?;
                let param_type = if self.match_token(&[Token::Colon]) {
                    Some(self.parse_type()?)
                } else {
                    None
                };
                params.push(Param {
                    name: param_name,
                    type_annotation: param_type,
                });
                
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        
        self.consume(&Token::RightParen, "Expected ')' after parameters")?;
        
        let return_type = if self.match_token(&[Token::Arrow]) {
            Some(self.parse_type()?)
        } else {
            None
        };
        
        // Functions must have braces for now
        self.consume(&Token::LeftBrace, "Expected '{' after function signature")?;
        let body = self.block_with_brace_consumed()?;
        
        Ok(Stmt::Expr(Expr::Function {
            name,
            params,
            return_type,
            body: Box::new(body),
        }))
    }
    
    fn parse_type(&mut self) -> Result<Type, TogError> {
        if self.match_token(&[Token::Keyword(Keyword::Int)]) {
            Ok(Type::Int)
        } else if self.match_token(&[Token::Keyword(Keyword::Float)]) {
            Ok(Type::Float)
        } else if self.match_token(&[Token::Keyword(Keyword::String)]) {
            Ok(Type::String)
        } else if self.match_token(&[Token::Keyword(Keyword::Bool)]) {
            Ok(Type::Bool)
        } else if self.match_token(&[Token::Keyword(Keyword::Array)]) {
            self.consume(&Token::LeftBracket, "Expected '[' after 'array'")?;
            let inner_type = self.parse_type()?;
            self.consume(&Token::RightBracket, "Expected ']' after array type")?;
            Ok(Type::Array(Box::new(inner_type)))
        } else if let Token::Identifier(name) = self.peek() {
            // Struct or Enum type name
            // We can't distinguish here, so we'll treat both as custom types
            // The type checker will validate later
            let name = name.clone();
            self.advance();
            // For now, assume it's a struct. The interpreter will handle enums.
            Ok(Type::Struct(name))
        } else {
            Err(TogError::ParseError(
                "Expected type".to_string(),
                0, 0
            ))
        }
    }
    
    fn statement(&mut self) -> Result<Stmt, TogError> {
        // Check if we're at end - return empty statement
        if self.is_at_end() {
            return Err(TogError::ParseError(
                "Unexpected end of file".to_string(),
                0, 0
            ));
        }
        
        // Check for assignment: identifier = expression or field_access = expression
        let current_pos = self.current;
        if current_pos + 1 < self.tokens.len() {
            if let Token::Identifier(_) = self.peek() {
                // Check for simple assignment: identifier = ...
                if matches!(&self.tokens[current_pos + 1], Token::Eq) {
                    if let Token::Identifier(name) = self.peek() {
                        let var_name = name.clone();
                        self.advance(); // consume identifier
                        self.advance(); // consume =
                        let value = self.expression()?;
                        return Ok(Stmt::Assign {
                            name: var_name,
                            value,
                        });
                    }
                }
                // Check for field assignment: identifier.field = ...
                else if current_pos + 3 < self.tokens.len() {
                    if matches!(&self.tokens[current_pos + 1], Token::Dot) {
                        if let Token::Identifier(_) = &self.tokens[current_pos + 2] {
                            if matches!(&self.tokens[current_pos + 3], Token::Eq) {
                                // Parse: identifier.field = value
                                let obj_name = if let Token::Identifier(name) = self.peek() {
                                    name.clone()
                                } else {
                                    return Err(TogError::ParseError("Expected identifier".to_string(), 0, 0));
                                };
                                self.advance(); // consume identifier
                                self.consume(&Token::Dot, "Expected '.'")?;
                                let field_name = self.consume_identifier()?;
                                self.consume(&Token::Eq, "Expected '='")?;
                                let value = self.expression()?;
                                return Ok(Stmt::AssignField {
                                    object: Box::new(Expr::Variable(obj_name)),
                                    field: field_name,
                                    value,
                                });
                            }
                        }
                    }
                }
            }
        }
        
        // print is now a function call, not a statement
        if self.match_token(&[Token::Keyword(Keyword::Return)]) {
            let value = if !self.check(&Token::Semicolon) && !self.is_at_end() {
                Some(self.expression()?)
            } else {
                None
            };
            Ok(Stmt::Return(value))
        } else if self.match_token(&[Token::Keyword(Keyword::Break)]) {
            Ok(Stmt::Break)
        } else if self.match_token(&[Token::Keyword(Keyword::Continue)]) {
            Ok(Stmt::Continue)
        } else if self.match_token(&[Token::LeftBrace]) {
            Ok(Stmt::Expr(self.block()?))
        } else if self.match_token(&[Token::Keyword(Keyword::If)]) {
            self.if_statement()
        } else if self.match_token(&[Token::Keyword(Keyword::While)]) {
            self.while_statement()
        } else if self.match_token(&[Token::Keyword(Keyword::For)]) {
            self.for_statement()
        } else {
            let expr = self.expression()?;
            Ok(Stmt::Expr(expr))
        }
    }
    
    fn if_statement(&mut self) -> Result<Stmt, TogError> {
        let condition = self.expression()?;
        let then_branch = Box::new(self.block()?);
        
        let else_branch = if self.match_token(&[Token::Keyword(Keyword::Else)]) {
            Some(Box::new(self.block()?))
        } else {
            None
        };
        
        Ok(Stmt::Expr(Expr::If {
            condition: Box::new(condition),
            then_branch,
            else_branch,
        }))
    }
    
    fn while_statement(&mut self) -> Result<Stmt, TogError> {
        let condition = self.expression()?;
        let body = Box::new(self.block()?);
        
        Ok(Stmt::Expr(Expr::While {
            condition: Box::new(condition),
            body,
        }))
    }
    
    fn for_statement(&mut self) -> Result<Stmt, TogError> {
        // Parse: for variable in iterable { body }
        let variable = self.consume_identifier()?;
        
        // Expect 'in' keyword
        self.consume(&Token::Keyword(Keyword::In), "Expected 'in' after loop variable")?;
        
        let iterable = Box::new(self.expression()?);
        let body = Box::new(self.block()?);
        
        Ok(Stmt::Expr(Expr::For {
            variable,
            iterable,
            body,
        }))
    }
    
    fn block(&mut self) -> Result<Expr, TogError> {
        if !self.match_token(&[Token::LeftBrace]) {
            // Single expression (no braces)
            return Ok(self.expression()?);
        }
        
        self.block_with_brace_consumed()
    }
    
    fn block_with_brace_consumed(&mut self) -> Result<Expr, TogError> {
        let mut statements = Vec::new();
        
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        
        self.consume(&Token::RightBrace, "Expected '}' after block")?;
        
        Ok(Expr::Block(statements))
    }
    
    fn expression(&mut self) -> Result<Expr, TogError> {
        self.assignment()
    }
    
    fn assignment(&mut self) -> Result<Expr, TogError> {
        self.or()
    }
    
    fn or(&mut self) -> Result<Expr, TogError> {
        let mut expr = self.and()?;
        
        while self.match_token(&[Token::Or]) {
            let op = BinaryOp::Or;
            let right = self.and()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn and(&mut self) -> Result<Expr, TogError> {
        let mut expr = self.equality()?;
        
        while self.match_token(&[Token::And]) {
            let op = BinaryOp::And;
            let right = self.equality()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn equality(&mut self) -> Result<Expr, TogError> {
        let mut expr = self.comparison()?;
        
        while self.match_token(&[Token::EqEq, Token::Ne]) {
            let op = match self.previous().clone() {
                Token::EqEq => BinaryOp::Eq,
                Token::Ne => BinaryOp::Ne,
                _ => unreachable!(),
            };
            let right = self.comparison()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn comparison(&mut self) -> Result<Expr, TogError> {
        let mut expr = self.term()?;
        
        while self.match_token(&[Token::Gt, Token::Ge, Token::Lt, Token::Le]) {
            let op = match self.previous().clone() {
                Token::Gt => BinaryOp::Gt,
                Token::Ge => BinaryOp::Ge,
                Token::Lt => BinaryOp::Lt,
                Token::Le => BinaryOp::Le,
                _ => unreachable!(),
            };
            let right = self.term()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn term(&mut self) -> Result<Expr, TogError> {
        let mut expr = self.factor()?;
        
        while self.match_token(&[Token::Plus, Token::Minus]) {
            let op = match self.previous().clone() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };
            let right = self.factor()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn factor(&mut self) -> Result<Expr, TogError> {
        let mut expr = self.unary()?;
        
        while self.match_token(&[Token::Star, Token::Slash, Token::Percent]) {
            let op = match self.previous().clone() {
                Token::Star => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
                Token::Percent => BinaryOp::Mod,
                _ => unreachable!(),
            };
            let right = self.unary()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        
        Ok(expr)
    }
    
    fn unary(&mut self) -> Result<Expr, TogError> {
        if self.match_token(&[Token::Not, Token::Minus]) {
            let op = match self.previous().clone() {
                Token::Not => UnaryOp::Not,
                Token::Minus => UnaryOp::Neg,
                _ => unreachable!(),
            };
            let expr = self.unary()?;
            return Ok(Expr::UnaryOp {
                op,
                expr: Box::new(expr),
            });
        }
        
        self.call()
    }
    
    fn match_expression(&mut self) -> Result<Expr, TogError> {
        // Note: match keyword was already consumed if called from statement()
        // But if called from unary(), it was also consumed
        // Parse the expression being matched (use or() to avoid recursion)
        let expr = Box::new(self.or()?);

        self.consume(&Token::LeftBrace, "Expected '{' after match expression")?;
        
        let mut arms = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            let pattern = self.parse_pattern()?;
            self.consume(&Token::FatArrow, "Expected '=>' after pattern")?;
            let body = self.expression()?;
            
            // Optional comma between arms
            let _ = self.match_token(&[Token::Comma]);
            
            arms.push(MatchArm { pattern, body });
        }
        
        self.consume(&Token::RightBrace, "Expected '}' after match arms")?;
        
        Ok(Expr::Match { expr, arms })
    }
    
    fn parse_pattern(&mut self) -> Result<Pattern, TogError> {
        if self.match_token(&[Token::Keyword(Keyword::None)]) {
            return Ok(Pattern::Literal(Literal::None));
        }
        if self.check(&Token::Bool(true)) {
            self.advance();
            return Ok(Pattern::Literal(Literal::Bool(true)));
        }
        if self.check(&Token::Bool(false)) {
            self.advance();
            return Ok(Pattern::Literal(Literal::Bool(false)));
        }
        if let Token::Int(val) = self.peek() {
            let val = *val;
            self.advance();
            return Ok(Pattern::Literal(Literal::Int(val)));
        }
        if let Token::Float(val) = self.peek() {
            let val = *val;
            self.advance();
            return Ok(Pattern::Literal(Literal::Float(val)));
        }
        if let Token::String(val) = self.peek() {
            let val = val.clone();
            self.advance();
            return Ok(Pattern::Literal(Literal::String(val)));
        }
        if let Token::Identifier(name) = self.peek() {
            let name = name.clone();
            self.advance();
            if name == "_" {
                return Ok(Pattern::Wildcard);
            }
            return Ok(Pattern::Variable(name));
        }
        
        Err(TogError::ParseError(
            "Expected pattern".to_string(),
            0, 0
        ))
    }
    
    fn call(&mut self) -> Result<Expr, TogError> {
        let mut expr = self.primary()?;
        
        loop {
            if self.match_token(&[Token::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else if self.match_token(&[Token::LeftBracket]) {
                // Array indexing
                let index = self.expression()?;
                self.consume(&Token::RightBracket, "Expected ']' after index")?;
                expr = Expr::Index {
                    array: Box::new(expr),
                    index: Box::new(index),
                };
            } else if self.match_token(&[Token::Dot]) {
                // Field access
                let field_name = self.consume_identifier()?;
                expr = Expr::FieldAccess {
                    object: Box::new(expr),
                    field: field_name,
                };
            } else {
                break;
            }
        }
        
        Ok(expr)
    }
    
    fn finish_call(&mut self, callee: Expr) -> Result<Expr, TogError> {
        let mut args = Vec::new();
        
        if !self.check(&Token::RightParen) {
            loop {
                args.push(self.expression()?);
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        
        self.consume(&Token::RightParen, "Expected ')' after arguments")?;
        
        Ok(Expr::Call {
            callee: Box::new(callee),
            args,
        })
    }
    
    fn primary(&mut self) -> Result<Expr, TogError> {
        if self.match_token(&[Token::Keyword(Keyword::None)]) {
            return Ok(Expr::Literal(Literal::None));
        }
        if self.match_token(&[Token::Keyword(Keyword::Match)]) {
            return self.match_expression();
        }

        if let Some(token) = self.tokens.get(self.current) {
            match token.clone() {
                Token::Int(val) => {
                    self.advance();
                    return Ok(Expr::Literal(Literal::Int(val)));
                },
                Token::Float(val) => {
                    self.advance();
                    return Ok(Expr::Literal(Literal::Float(val)));
                },
                Token::String(val) => {
                    self.advance();
                    return Ok(Expr::Literal(Literal::String(val)));
                },
                Token::InterpolatedString(val) => {
                    self.advance();
                    return Ok(Expr::Literal(Literal::String(val)));
                },
                Token::Bool(val) => {
                    self.advance();
                    return Ok(Expr::Literal(Literal::Bool(val)));
                },
                Token::Identifier(name) => {
                    // Check for struct literal: Point { ... }
                    if self.check_ahead(1, &Token::LeftBrace) {
                        return self.struct_literal();
                    }

                    if name == "_" {
                        // A wildcard `_` is not a valid expression on its own.
                        // It's only valid as a pattern in a match arm.
                        return Err(TogError::ParseError("Wildcard `_` can only be used as a pattern in a match arm.".to_string(), 0, 0));
                    } else {
                        self.advance();
                        return Ok(Expr::Variable(name.clone()));
                    }
                },
                Token::LeftBracket => {
                    self.advance(); // consume '['
                    let elements = self.array()?;
                    // array() already consumes the ']'
                    return Ok(elements);
                },
                Token::LeftParen => {
                    self.advance(); // consume '('
                    let expr = self.expression()?;
                    self.consume(&Token::RightParen, "Expected ')' after expression")?;
                    return Ok(expr);
                },
                _ => {}
            }
        }
        
        Err(TogError::ParseError(
            "Expected expression".to_string(),
            0, 0
        ))
    }
    
    fn array(&mut self) -> Result<Expr, TogError> {
        let mut elements = Vec::new();
        
        if !self.check(&Token::RightBracket) {
            loop {
                elements.push(self.expression()?);
                if !self.match_token(&[Token::Comma]) {
                    break;
                }
            }
        }
        
        self.consume(&Token::RightBracket, "Expected ']' after array")?;
        
        Ok(Expr::Literal(Literal::Array(elements)))
    }
    
    fn struct_literal(&mut self) -> Result<Expr, TogError> {
        let name = self.consume_identifier()?;
        self.consume(&Token::LeftBrace, "Expected '{' after struct name")?;
        let mut fields = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            let field_name = self.consume_identifier()?;
            self.consume(&Token::Colon, "Expected ':' after field name in struct literal")?;
            let field_value = self.expression()?;
            fields.push((field_name, field_value));
            if !self.match_token(&[Token::Comma]) {
                break;
            }
        }
        self.consume(&Token::RightBrace, "Expected '}' after struct literal")?;

        Ok(Expr::StructLiteral { name, fields })
    }

    // Helper methods
    fn match_token(&mut self, tokens: &[Token]) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }
    
    
    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            return false;
        }
        match (token, &self.tokens[self.current]) {
            (Token::Int(_), Token::Int(_)) => true,
            (Token::Float(_), Token::Float(_)) => true,
            (Token::String(_), Token::String(_)) => true,
            (Token::InterpolatedString(_), Token::InterpolatedString(_)) => true,
            (Token::String(_), Token::InterpolatedString(_)) => true, // Allow matching
            (Token::InterpolatedString(_), Token::String(_)) => true, // Allow matching
            (Token::Bool(a), Token::Bool(b)) => a == b,
            (Token::Keyword(k1), Token::Keyword(k2)) => k1 == k2,
            (Token::Identifier(_), Token::Identifier(_)) => true,
            (t1, t2) => std::mem::discriminant(t1) == std::mem::discriminant(t2),
        }
    }
    
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }
    
    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }
    
    fn check_ahead(&self, distance: usize, token: &Token) -> bool {
        if self.current + distance >= self.tokens.len() {
            return false;
        }
        let future_token = &self.tokens[self.current + distance];
        match (token, future_token) {
            (Token::Int(_), Token::Int(_)) => true,
            (Token::Float(_), Token::Float(_)) => true,
            (Token::String(_), Token::String(_)) => true,
            (Token::InterpolatedString(_), Token::InterpolatedString(_)) => true,
            (Token::Identifier(_), Token::Identifier(_)) => true,
            _ => token == future_token,
        }
    }

    fn peek(&self) -> &Token {
        if self.current >= self.tokens.len() {
            &self.tokens[self.tokens.len() - 1] // Return last token (should be Eof)
        } else {
            &self.tokens[self.current]
        }
    }
    
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
    
    fn consume(&mut self, token: &Token, message: &str) -> Result<(), TogError> {
        if self.check(token) {
            self.advance();
            Ok(())
        } else {
            Err(TogError::ParseError(
                format!("{}: expected {:?}", message, token),
                0, 0
            ))
        }
    }
    
    fn consume_identifier(&mut self) -> Result<String, TogError> {
        if let Token::Identifier(name) = self.peek() {
            let name = name.clone();
            self.advance();
            Ok(name)
        } else {
            Err(TogError::ParseError(
                "Expected identifier".to_string(),
                0, 0
            ))
        }
    }
}

