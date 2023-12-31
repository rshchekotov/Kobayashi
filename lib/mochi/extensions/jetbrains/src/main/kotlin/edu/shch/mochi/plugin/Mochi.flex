package edu.shch.mochi.plugin;

import com.intellij.lexer.FlexLexer;
import com.intellij.psi.tree.IElementType;
import com.intellij.psi.TokenType;
import edu.shch.mochi.plugin.psi.MochiTypes;

// https://github.com/JetBrains/intellij-deps-jflex/blob/master/docs/md/example.md
// https://github.com/pest-parser/intellij-pest/blob/master/grammar/pest.flex

%%

%class MochiLexer
%implements FlexLexer
%unicode
%function advance
%type IElementType
%eof{ return;
%eof}

/* Special Values */
WHITESPACE=[ \t\r\n]+
COMMENT="/*".*"*/"
LINE_COMMENT="//".*\n

/* Operators */
ASSIGNMENT_OP="->"
ARRAY_SEP=","

/* Brackets */
LBRACE="{"
RBRACE="}"
LBRACKET="["
RBRACKET="]"

/* Text Types */
IDENTIFIER=[a-zA-Z][a-zA-Z0-9_]+
STRING_LITERAL="\""([^\\\"])*"\""

/* Other Types */
INTEGER_LITERAL=[0-9]+
BOOLEAN_LITERAL="true"|"false"
COLOR_LITERAL="#"[0-9a-fA-F]{1,6}

%state EXPECT_VALUE, EXPECT_ARRAY_VALUE

%%

<YYINITIAL> {
    {LINE_COMMENT}    { yybegin(YYINITIAL); return MochiTypes.COMMENT; }
    {COMMENT}         { yybegin(YYINITIAL); return MochiTypes.COMMENT; }

    {WHITESPACE}      { yybegin(YYINITIAL); return TokenType.WHITE_SPACE; }
    {ASSIGNMENT_OP}   { yybegin(EXPECT_VALUE); return MochiTypes.ASSIGNMENT_OP; }

    {LBRACE}          { yybegin(YYINITIAL); return MochiTypes.LBRACE; }
    {RBRACE}          { yybegin(YYINITIAL); return MochiTypes.RBRACE; }

    {STRING_LITERAL}  { yybegin(YYINITIAL); return MochiTypes.STRING; }
    {COLOR_LITERAL}   { yybegin(YYINITIAL); return MochiTypes.COLOR; }
    {BOOLEAN_LITERAL} { yybegin(YYINITIAL); return MochiTypes.BOOLEAN; }
    {INTEGER_LITERAL} { yybegin(YYINITIAL); return MochiTypes.INTEGER; }
    {IDENTIFIER}      { yybegin(YYINITIAL); return MochiTypes.IDENTIFIER; }
}

<EXPECT_VALUE> {
    {WHITESPACE}      { yybegin(EXPECT_VALUE); return TokenType.WHITE_SPACE; }
    {LBRACKET}        { yybegin(EXPECT_ARRAY_VALUE); return MochiTypes.LBRACKET; }
    {RBRACKET}        { yybegin(YYINITIAL); return TokenType.BAD_CHARACTER; }

    {STRING_LITERAL}  { yybegin(YYINITIAL); return MochiTypes.STRING; }
    {COLOR_LITERAL}   { yybegin(YYINITIAL); return MochiTypes.COLOR; }
    {BOOLEAN_LITERAL} { yybegin(YYINITIAL); return MochiTypes.BOOLEAN; }
    {INTEGER_LITERAL} { yybegin(YYINITIAL); return MochiTypes.INTEGER; }
    {IDENTIFIER}      { yybegin(YYINITIAL); return MochiTypes.ENUM_VALUE; }
}

// Support 1-dimensional arrays
<EXPECT_ARRAY_VALUE> {
    {WHITESPACE}      { yybegin(EXPECT_ARRAY_VALUE); return TokenType.WHITE_SPACE; }
    {ARRAY_SEP}       { yybegin(EXPECT_ARRAY_VALUE); return MochiTypes.ARRAY_SEP; }
    {LBRACKET}        { yybegin(YYINITIAL); return TokenType.BAD_CHARACTER; }
    {RBRACKET}        { yybegin(YYINITIAL); return MochiTypes.RBRACKET; }

    {STRING_LITERAL}  { yybegin(EXPECT_ARRAY_VALUE); return MochiTypes.STRING; }
    {COLOR_LITERAL}   { yybegin(EXPECT_ARRAY_VALUE); return MochiTypes.COLOR; }
    {BOOLEAN_LITERAL} { yybegin(EXPECT_ARRAY_VALUE); return MochiTypes.BOOLEAN; }
    {INTEGER_LITERAL} { yybegin(EXPECT_ARRAY_VALUE); return MochiTypes.INTEGER; }
    {IDENTIFIER}      { yybegin(EXPECT_ARRAY_VALUE); return MochiTypes.ENUM_VALUE; }
}

{WHITESPACE}+  { yybegin(YYINITIAL); return TokenType.WHITE_SPACE; }

[^]            { return TokenType.BAD_CHARACTER; }