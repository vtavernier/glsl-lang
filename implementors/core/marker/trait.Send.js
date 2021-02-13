(function() {var implementors = {};
implementors["glsl_lang"] = [{"text":"impl Send for Path","synthetic":true,"types":[]},{"text":"impl Send for IdentifierData","synthetic":true,"types":[]},{"text":"impl Send for TypeNameData","synthetic":true,"types":[]},{"text":"impl Send for TypeSpecifierNonArray","synthetic":true,"types":[]},{"text":"impl Send for TypeSpecifier","synthetic":true,"types":[]},{"text":"impl Send for StructSpecifier","synthetic":true,"types":[]},{"text":"impl Send for StructFieldSpecifier","synthetic":true,"types":[]},{"text":"impl Send for ArrayedIdentifier","synthetic":true,"types":[]},{"text":"impl Send for TypeQualifier","synthetic":true,"types":[]},{"text":"impl Send for TypeQualifierSpec","synthetic":true,"types":[]},{"text":"impl Send for StorageQualifier","synthetic":true,"types":[]},{"text":"impl Send for LayoutQualifier","synthetic":true,"types":[]},{"text":"impl Send for LayoutQualifierSpec","synthetic":true,"types":[]},{"text":"impl Send for PrecisionQualifier","synthetic":true,"types":[]},{"text":"impl Send for InterpolationQualifier","synthetic":true,"types":[]},{"text":"impl Send for FullySpecifiedType","synthetic":true,"types":[]},{"text":"impl Send for ArraySpecifier","synthetic":true,"types":[]},{"text":"impl Send for ArraySpecifierDimension","synthetic":true,"types":[]},{"text":"impl Send for DeclarationData","synthetic":true,"types":[]},{"text":"impl Send for Block","synthetic":true,"types":[]},{"text":"impl Send for FunIdentifier","synthetic":true,"types":[]},{"text":"impl Send for FunctionPrototypeData","synthetic":true,"types":[]},{"text":"impl Send for FunctionParameterDeclarationData","synthetic":true,"types":[]},{"text":"impl Send for FunctionParameterDeclarator","synthetic":true,"types":[]},{"text":"impl Send for InitDeclaratorList","synthetic":true,"types":[]},{"text":"impl Send for SingleDeclaration","synthetic":true,"types":[]},{"text":"impl Send for SingleDeclarationNoType","synthetic":true,"types":[]},{"text":"impl Send for Initializer","synthetic":true,"types":[]},{"text":"impl Send for Expr","synthetic":true,"types":[]},{"text":"impl Send for UnaryOp","synthetic":true,"types":[]},{"text":"impl Send for BinaryOp","synthetic":true,"types":[]},{"text":"impl Send for AssignmentOp","synthetic":true,"types":[]},{"text":"impl Send for TranslationUnit","synthetic":true,"types":[]},{"text":"impl Send for ExternalDeclarationData","synthetic":true,"types":[]},{"text":"impl Send for FunctionDefinitionData","synthetic":true,"types":[]},{"text":"impl Send for CompoundStatementData","synthetic":true,"types":[]},{"text":"impl Send for StatementData","synthetic":true,"types":[]},{"text":"impl Send for ExprStatement","synthetic":true,"types":[]},{"text":"impl Send for SelectionStatement","synthetic":true,"types":[]},{"text":"impl Send for Condition","synthetic":true,"types":[]},{"text":"impl Send for SelectionRestStatement","synthetic":true,"types":[]},{"text":"impl Send for SwitchStatement","synthetic":true,"types":[]},{"text":"impl Send for CaseLabel","synthetic":true,"types":[]},{"text":"impl Send for IterationStatement","synthetic":true,"types":[]},{"text":"impl Send for ForInitStatement","synthetic":true,"types":[]},{"text":"impl Send for ForRestStatement","synthetic":true,"types":[]},{"text":"impl Send for JumpStatement","synthetic":true,"types":[]},{"text":"impl Send for PreprocessorData","synthetic":true,"types":[]},{"text":"impl Send for PreprocessorDefine","synthetic":true,"types":[]},{"text":"impl Send for PreprocessorElseIf","synthetic":true,"types":[]},{"text":"impl Send for PreprocessorError","synthetic":true,"types":[]},{"text":"impl Send for PreprocessorIf","synthetic":true,"types":[]},{"text":"impl Send for PreprocessorIfDef","synthetic":true,"types":[]},{"text":"impl Send for PreprocessorIfNDef","synthetic":true,"types":[]},{"text":"impl Send for PreprocessorInclude","synthetic":true,"types":[]},{"text":"impl Send for PreprocessorLine","synthetic":true,"types":[]},{"text":"impl Send for PreprocessorPragma","synthetic":true,"types":[]},{"text":"impl Send for PreprocessorUndef","synthetic":true,"types":[]},{"text":"impl Send for PreprocessorVersion","synthetic":true,"types":[]},{"text":"impl Send for PreprocessorVersionProfile","synthetic":true,"types":[]},{"text":"impl Send for PreprocessorExtension","synthetic":true,"types":[]},{"text":"impl Send for PreprocessorExtensionName","synthetic":true,"types":[]},{"text":"impl Send for PreprocessorExtensionBehavior","synthetic":true,"types":[]},{"text":"impl Send for CommentData","synthetic":true,"types":[]},{"text":"impl&lt;'i&gt; !Send for PreprocessorToken&lt;'i&gt;","synthetic":true,"types":[]},{"text":"impl Send for PreprocessorTokenKind","synthetic":true,"types":[]},{"text":"impl&lt;'i&gt; !Send for Token&lt;'i&gt;","synthetic":true,"types":[]},{"text":"impl Send for TokenKind","synthetic":true,"types":[]},{"text":"impl&lt;'i&gt; !Send for LexerStage&lt;'i&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'i&gt; !Send for Lexer&lt;'i&gt;","synthetic":true,"types":[]},{"text":"impl Send for LexicalError","synthetic":true,"types":[]},{"text":"impl&lt;'i&gt; !Send for __Symbol&lt;'i&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'i&gt; Send for __StateMachine&lt;'i&gt;","synthetic":true,"types":[]},{"text":"impl Send for TranslationUnitParser","synthetic":true,"types":[]},{"text":"impl Send for ParseOptions","synthetic":true,"types":[]},{"text":"impl !Send for ParseContext","synthetic":true,"types":[]},{"text":"impl !Send for ParseContextData","synthetic":true,"types":[]},{"text":"impl Send for IdentifierContext","synthetic":true,"types":[]},{"text":"impl Send for GlslTypeTablePolicy","synthetic":true,"types":[]},{"text":"impl Send for IndentStyle","synthetic":true,"types":[]},{"text":"impl Send for Whitespace","synthetic":true,"types":[]},{"text":"impl Send for FormattingSettings","synthetic":true,"types":[]},{"text":"impl&lt;'s&gt; Send for FormattingState&lt;'s&gt;","synthetic":true,"types":[]},{"text":"impl Send for DEFAULT_SETTINGS","synthetic":true,"types":[]},{"text":"impl Send for Visit","synthetic":true,"types":[]}];
implementors["lang_util"] = [{"text":"impl Send for LexerPosition","synthetic":true,"types":[]},{"text":"impl Send for NodeSpan","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; Send for Node&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;'a, T:&nbsp;?Sized&gt; Send for NodeDisplayWrapper&lt;'a, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["lang_util_derive"] = [{"text":"impl Send for DisplayFieldOpts","synthetic":true,"types":[]},{"text":"impl Send for DisplayVariantOpts","synthetic":true,"types":[]},{"text":"impl !Send for NodeDisplayField","synthetic":true,"types":[]},{"text":"impl !Send for NodeDisplayVariant","synthetic":true,"types":[]},{"text":"impl Send for NodeDisplay","synthetic":true,"types":[]},{"text":"impl !Send for NodeContentOpts","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()