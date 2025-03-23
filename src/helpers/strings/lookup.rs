//! # Lookup
//!
//! Lookup tables for string helper

pub static HTML_ENTITIES_TABLE: [(&str, &str); 1452] = [
    ("AElig", "\u{00C6}"),                // LATIN CAPITAL LETTER AE
    ("AMP", "\u{0026}"),                  // AMPERSAND
    ("Aacute", "\u{00C1}"),               // LATIN CAPITAL LETTER A WITH ACUTE
    ("Abreve", "\u{0102}"),               // LATIN CAPITAL LETTER A WITH BREVE
    ("Acirc", "\u{00C2}"),                // LATIN CAPITAL LETTER A WITH CIRCUMFLEX
    ("Acy", "\u{0410}"),                  // CYRILLIC CAPITAL LETTER A
    ("Afr", "\u{1D504}"),                 // MATHEMATICAL FRAKTUR CAPITAL A
    ("Agrave", "\u{00C0}"),               // LATIN CAPITAL LETTER A WITH GRAVE
    ("Alpha", "\u{0391}"),                // GREEK CAPITAL LETTER ALPHA
    ("Amacr", "\u{0100}"),                // LATIN CAPITAL LETTER A WITH MACRON
    ("And", "\u{2A53}"),                  // DOUBLE LOGICAL AND
    ("Aogon", "\u{0104}"),                // LATIN CAPITAL LETTER A WITH OGONEK
    ("Aopf", "\u{1D538}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL A
    ("ApplyFunction", "\u{2061}"),        // FUNCTION APPLICATION
    ("Aring", "\u{00C5}"),                // LATIN CAPITAL LETTER A WITH RING ABOVE
    ("Ascr", "\u{1D49C}"),                // MATHEMATICAL SCRIPT CAPITAL A
    ("Atilde", "\u{00C3}"),               // LATIN CAPITAL LETTER A WITH TILDE
    ("Auml", "\u{00C4}"),                 // LATIN CAPITAL LETTER A WITH DIAERESIS
    ("Barv", "\u{2AE7}"),                 // SHORT DOWN TACK WITH OVERBAR
    ("Barwed", "\u{2306}"),               // PERSPECTIVE
    ("Bcy", "\u{0411}"),                  // CYRILLIC CAPITAL LETTER BE
    ("Beta", "\u{0392}"),                 // GREEK CAPITAL LETTER BETA
    ("Bfr", "\u{1D505}"),                 // MATHEMATICAL FRAKTUR CAPITAL B
    ("Bopf", "\u{1D539}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL B
    ("CHcy", "\u{0427}"),                 // CYRILLIC CAPITAL LETTER CHE
    ("Cacute", "\u{0106}"),               // LATIN CAPITAL LETTER C WITH ACUTE
    ("Cap", "\u{22D2}"),                  // DOUBLE INTERSECTION
    ("CapitalDifferentialD", "\u{2145}"), // DOUBLE-STRUCK ITALIC CAPITAL D
    ("Ccaron", "\u{010C}"),               // LATIN CAPITAL LETTER C WITH CARON
    ("Ccedil", "\u{00C7}"),               // LATIN CAPITAL LETTER C WITH CEDILLA
    ("Ccirc", "\u{0108}"),                // LATIN CAPITAL LETTER C WITH CIRCUMFLEX
    ("Cconint", "\u{2230}"),              // VOLUME INTEGRAL
    ("Cdot", "\u{010A}"),                 // LATIN CAPITAL LETTER C WITH DOT ABOVE
    ("Cfr", "\u{212D}"),                  // BLACK-LETTER CAPITAL C
    ("Chi", "\u{03A7}"),                  // GREEK CAPITAL LETTER CHI
    ("Colon", "\u{2237}"),                // PROPORTION
    ("Colone", "\u{2A74}"),               // DOUBLE COLON EQUAL
    ("Conint", "\u{222F}"),               // SURFACE INTEGRAL
    ("Copf", "\u{2102}"),                 // DOUBLE-STRUCK CAPITAL C
    ("Cross", "\u{2A2F}"),                // VECTOR OR CROSS PRODUCT
    ("Cscr", "\u{1D49E}"),                // MATHEMATICAL SCRIPT CAPITAL C
    ("Cup", "\u{22D3}"),                  // DOUBLE UNION
    ("DDotrahd", "\u{2911}"),             // RIGHTWARDS ARROW WITH DOTTED STEM
    ("DJcy", "\u{0402}"),                 // CYRILLIC CAPITAL LETTER DJE
    ("DScy", "\u{0405}"),                 // CYRILLIC CAPITAL LETTER DZE
    ("DZcy", "\u{040F}"),                 // CYRILLIC CAPITAL LETTER DZHE
    ("Dagger", "\u{2021}"),               // DOUBLE DAGGER
    ("Darr", "\u{21A1}"),                 // DOWNWARDS TWO HEADED ARROW
    ("Dashv", "\u{2AE4}"),                // VERTICAL BAR DOUBLE LEFT TURNSTILE
    ("Dcaron", "\u{010E}"),               // LATIN CAPITAL LETTER D WITH CARON
    ("Dcy", "\u{0414}"),                  // CYRILLIC CAPITAL LETTER DE
    ("Delta", "\u{0394}"),                // GREEK CAPITAL LETTER DELTA
    ("Dfr", "\u{1D507}"),                 // MATHEMATICAL FRAKTUR CAPITAL D
    ("DifferentialD", "\u{2146}"),        // DOUBLE-STRUCK ITALIC SMALL D
    ("Dopf", "\u{1D53B}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL D
    ("Dot", "\u{00A8}"),                  // DIAERESIS
    ("DotDot", "\u{20DC}"),               // COMBINING FOUR DOTS ABOVE
    ("DownArrowBar", "\u{2913}"),         // DOWNWARDS ARROW TO BAR
    ("DownBreve", "\u{0311}"),            // COMBINING INVERTED BREVE
    ("DownLeftRightVector", "\u{2950}"),  // LEFT BARB DOWN RIGHT BARB DOWN HARPOON
    ("DownLeftTeeVector", "\u{295E}"),    // LEFTWARDS HARPOON WITH BARB DOWN FROM BAR
    ("DownLeftVectorBar", "\u{2956}"),    // LEFTWARDS HARPOON WITH BARB DOWN TO BAR
    ("DownRightTeeVector", "\u{295F}"),   // RIGHTWARDS HARPOON WITH BARB DOWN FROM BAR
    ("DownRightVectorBar", "\u{2957}"),   // RIGHTWARDS HARPOON WITH BARB DOWN TO BAR
    ("DownTeeArrow", "\u{21A7}"),         // DOWNWARDS ARROW FROM BAR
    ("Dscr", "\u{1D49F}"),                // MATHEMATICAL SCRIPT CAPITAL D
    ("Dstrok", "\u{0110}"),               // LATIN CAPITAL LETTER D WITH STROKE
    ("ENG", "\u{014A}"),                  // LATIN CAPITAL LETTER ENG
    ("ETH", "\u{00D0}"),                  // LATIN CAPITAL LETTER ETH
    ("Eacute", "\u{00C9}"),               // LATIN CAPITAL LETTER E WITH ACUTE
    ("Ecaron", "\u{011A}"),               // LATIN CAPITAL LETTER E WITH CARON
    ("Ecirc", "\u{00CA}"),                // LATIN CAPITAL LETTER E WITH CIRCUMFLEX
    ("Ecy", "\u{042D}"),                  // CYRILLIC CAPITAL LETTER E
    ("Edot", "\u{0116}"),                 // LATIN CAPITAL LETTER E WITH DOT ABOVE
    ("Efr", "\u{1D508}"),                 // MATHEMATICAL FRAKTUR CAPITAL E
    ("Egrave", "\u{00C8}"),               // LATIN CAPITAL LETTER E WITH GRAVE
    ("Emacr", "\u{0112}"),                // LATIN CAPITAL LETTER E WITH MACRON
    ("EmptySmallSquare", "\u{25FB}"),     // WHITE MEDIUM SQUARE
    ("EmptyVerySmallSquare", "\u{25AB}"), // WHITE SMALL SQUARE
    ("Eogon", "\u{0118}"),                // LATIN CAPITAL LETTER E WITH OGONEK
    ("Eopf", "\u{1D53C}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL E
    ("Epsilon", "\u{0395}"),              // GREEK CAPITAL LETTER EPSILON
    ("Equal", "\u{2A75}"),                // TWO CONSECUTIVE EQUALS SIGNS
    ("Escr", "\u{2130}"),                 // SCRIPT CAPITAL E
    ("Esim", "\u{2A73}"),                 // EQUALS SIGN ABOVE TILDE OPERATOR
    ("Eta", "\u{0397}"),                  // GREEK CAPITAL LETTER ETA
    ("Euml", "\u{00CB}"),                 // LATIN CAPITAL LETTER E WITH DIAERESIS
    ("ExponentialE", "\u{2147}"),         // DOUBLE-STRUCK ITALIC SMALL E
    ("Fcy", "\u{0424}"),                  // CYRILLIC CAPITAL LETTER EF
    ("Ffr", "\u{1D509}"),                 // MATHEMATICAL FRAKTUR CAPITAL F
    ("FilledSmallSquare", "\u{25FC}"),    // BLACK MEDIUM SQUARE
    ("Fopf", "\u{1D53D}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL F
    ("Fscr", "\u{2131}"),                 // SCRIPT CAPITAL F
    ("GJcy", "\u{0403}"),                 // CYRILLIC CAPITAL LETTER GJE
    ("GT", "\u{003E}"),                   // GREATER-THAN SIGN
    ("Gamma", "\u{0393}"),                // GREEK CAPITAL LETTER GAMMA
    ("Gammad", "\u{03DC}"),               // GREEK LETTER DIGAMMA
    ("Gbreve", "\u{011E}"),               // LATIN CAPITAL LETTER G WITH BREVE
    ("Gcedil", "\u{0122}"),               // LATIN CAPITAL LETTER G WITH CEDILLA
    ("Gcirc", "\u{011C}"),                // LATIN CAPITAL LETTER G WITH CIRCUMFLEX
    ("Gcy", "\u{0413}"),                  // CYRILLIC CAPITAL LETTER GHE
    ("Gdot", "\u{0120}"),                 // LATIN CAPITAL LETTER G WITH DOT ABOVE
    ("Gfr", "\u{1D50A}"),                 // MATHEMATICAL FRAKTUR CAPITAL G
    ("Gg", "\u{22D9}"),                   // VERY MUCH GREATER-THAN
    ("Gopf", "\u{1D53E}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL G
    ("GreaterGreater", "\u{2AA2}"),       // DOUBLE NESTED GREATER-THAN
    ("Gscr", "\u{1D4A2}"),                // MATHEMATICAL SCRIPT CAPITAL G
    ("Gt", "\u{226B}"),                   // MUCH GREATER-THAN
    ("HARDcy", "\u{042A}"),               // CYRILLIC CAPITAL LETTER HARD SIGN
    ("Hat", "\u{005E}"),                  // CIRCUMFLEX ACCENT
    ("Hcirc", "\u{0124}"),                // LATIN CAPITAL LETTER H WITH CIRCUMFLEX
    ("Hfr", "\u{210C}"),                  // BLACK-LETTER CAPITAL H
    ("Hstrok", "\u{0126}"),               // LATIN CAPITAL LETTER H WITH STROKE
    ("IEcy", "\u{0415}"),                 // CYRILLIC CAPITAL LETTER IE
    ("IJlig", "\u{0132}"),                // LATIN CAPITAL LIGATURE IJ
    ("IOcy", "\u{0401}"),                 // CYRILLIC CAPITAL LETTER IO
    ("Iacute", "\u{00CD}"),               // LATIN CAPITAL LETTER I WITH ACUTE
    ("Icirc", "\u{00CE}"),                // LATIN CAPITAL LETTER I WITH CIRCUMFLEX
    ("Icy", "\u{0418}"),                  // CYRILLIC CAPITAL LETTER I
    ("Idot", "\u{0130}"),                 // LATIN CAPITAL LETTER I WITH DOT ABOVE
    ("Igrave", "\u{00CC}"),               // LATIN CAPITAL LETTER I WITH GRAVE
    ("Imacr", "\u{012A}"),                // LATIN CAPITAL LETTER I WITH MACRON
    ("ImaginaryI", "\u{2148}"),           // DOUBLE-STRUCK ITALIC SMALL I
    ("Int", "\u{222C}"),                  // DOUBLE INTEGRAL
    ("InvisibleComma", "\u{2063}"),       // INVISIBLE SEPARATOR
    ("InvisibleTimes", "\u{2062}"),       // INVISIBLE TIMES
    ("Iogon", "\u{012E}"),                // LATIN CAPITAL LETTER I WITH OGONEK
    ("Iopf", "\u{1D540}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL I
    ("Iota", "\u{0399}"),                 // GREEK CAPITAL LETTER IOTA
    ("Iscr", "\u{2110}"),                 // SCRIPT CAPITAL I
    ("Itilde", "\u{0128}"),               // LATIN CAPITAL LETTER I WITH TILDE
    ("Iukcy", "\u{0406}"),                // CYRILLIC CAPITAL LETTER BYELORUSSIAN-UKRAINIAN I
    ("Iuml", "\u{00CF}"),                 // LATIN CAPITAL LETTER I WITH DIAERESIS
    ("Jcirc", "\u{0134}"),                // LATIN CAPITAL LETTER J WITH CIRCUMFLEX
    ("Jcy", "\u{0419}"),                  // CYRILLIC CAPITAL LETTER SHORT I
    ("Jfr", "\u{1D50D}"),                 // MATHEMATICAL FRAKTUR CAPITAL J
    ("Jopf", "\u{1D541}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL J
    ("Jscr", "\u{1D4A5}"),                // MATHEMATICAL SCRIPT CAPITAL J
    ("Jsercy", "\u{0408}"),               // CYRILLIC CAPITAL LETTER JE
    ("Jukcy", "\u{0404}"),                // CYRILLIC CAPITAL LETTER UKRAINIAN IE
    ("KHcy", "\u{0425}"),                 // CYRILLIC CAPITAL LETTER HA
    ("KJcy", "\u{040C}"),                 // CYRILLIC CAPITAL LETTER KJE
    ("Kappa", "\u{039A}"),                // GREEK CAPITAL LETTER KAPPA
    ("Kcedil", "\u{0136}"),               // LATIN CAPITAL LETTER K WITH CEDILLA
    ("Kcy", "\u{041A}"),                  // CYRILLIC CAPITAL LETTER KA
    ("Kfr", "\u{1D50E}"),                 // MATHEMATICAL FRAKTUR CAPITAL K
    ("Kopf", "\u{1D542}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL K
    ("Kscr", "\u{1D4A6}"),                // MATHEMATICAL SCRIPT CAPITAL K
    ("LJcy", "\u{0409}"),                 // CYRILLIC CAPITAL LETTER LJE
    ("LT", "\u{003C}"),                   // LESS-THAN SIGN
    ("Lacute", "\u{0139}"),               // LATIN CAPITAL LETTER L WITH ACUTE
    ("Lambda", "\u{039B}"),               // GREEK CAPITAL LETTER LAMDA
    ("Lang", "\u{27EA}"),                 // MATHEMATICAL LEFT DOUBLE ANGLE BRACKET
    ("Larr", "\u{219E}"),                 // LEFTWARDS TWO HEADED ARROW
    ("Lcaron", "\u{013D}"),               // LATIN CAPITAL LETTER L WITH CARON
    ("Lcedil", "\u{013B}"),               // LATIN CAPITAL LETTER L WITH CEDILLA
    ("Lcy", "\u{041B}"),                  // CYRILLIC CAPITAL LETTER EL
    ("LeftDownTeeVector", "\u{2961}"),    // DOWNWARDS HARPOON WITH BARB LEFT FROM BAR
    ("LeftDownVectorBar", "\u{2959}"),    // DOWNWARDS HARPOON WITH BARB LEFT TO BAR
    ("LeftRightVector", "\u{294E}"),      // LEFT BARB UP RIGHT BARB UP HARPOON
    ("LeftTeeArrow", "\u{21A4}"),         // LEFTWARDS ARROW FROM BAR
    ("LeftTeeVector", "\u{295A}"),        // LEFTWARDS HARPOON WITH BARB UP FROM BAR
    ("LeftTriangleBar", "\u{29CF}"),      // LEFT TRIANGLE BESIDE VERTICAL BAR
    ("LeftUpDownVector", "\u{2951}"),     // UP BARB LEFT DOWN BARB LEFT HARPOON
    ("LeftUpTeeVector", "\u{2960}"),      // UPWARDS HARPOON WITH BARB LEFT FROM BAR
    ("LeftUpVectorBar", "\u{2958}"),      // UPWARDS HARPOON WITH BARB LEFT TO BAR
    ("LeftVectorBar", "\u{2952}"),        // LEFTWARDS HARPOON WITH BARB UP TO BAR
    ("LessLess", "\u{2AA1}"),             // DOUBLE NESTED LESS-THAN
    ("Lfr", "\u{1D50F}"),                 // MATHEMATICAL FRAKTUR CAPITAL L
    ("Ll", "\u{22D8}"),                   // VERY MUCH LESS-THAN
    ("Lmidot", "\u{013F}"),               // LATIN CAPITAL LETTER L WITH MIDDLE DOT
    ("Lopf", "\u{1D543}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL L
    ("Lscr", "\u{2112}"),                 // SCRIPT CAPITAL L
    ("Lstrok", "\u{0141}"),               // LATIN CAPITAL LETTER L WITH STROKE
    ("Lt", "\u{226A}"),                   // MUCH LESS-THAN
    ("Map", "\u{2905}"),                  // RIGHTWARDS TWO-HEADED ARROW FROM BAR
    ("Mcy", "\u{041C}"),                  // CYRILLIC CAPITAL LETTER EM
    ("MediumSpace", "\u{205F}"),          // MEDIUM MATHEMATICAL SPACE
    ("Mfr", "\u{1D510}"),                 // MATHEMATICAL FRAKTUR CAPITAL M
    ("Mopf", "\u{1D544}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL M
    ("Mu", "\u{039C}"),                   // GREEK CAPITAL LETTER MU
    ("NJcy", "\u{040A}"),                 // CYRILLIC CAPITAL LETTER NJE
    ("Nacute", "\u{0143}"),               // LATIN CAPITAL LETTER N WITH ACUTE
    ("Ncaron", "\u{0147}"),               // LATIN CAPITAL LETTER N WITH CARON
    ("Ncedil", "\u{0145}"),               // LATIN CAPITAL LETTER N WITH CEDILLA
    ("Ncy", "\u{041D}"),                  // CYRILLIC CAPITAL LETTER EN
    ("NewLine", "\u{000A}"),              // LINE FEED (LF)
    ("Nfr", "\u{1D511}"),                 // MATHEMATICAL FRAKTUR CAPITAL N
    ("NoBreak", "\u{2060}"),              // WORD JOINER
    ("Nopf", "\u{2115}"),                 // DOUBLE-STRUCK CAPITAL N
    ("Not", "\u{2AEC}"),                  // DOUBLE STROKE NOT SIGN
    ("NotCupCap", "\u{226D}"),            // NOT EQUIVALENT TO
    ("Nscr", "\u{1D4A9}"),                // MATHEMATICAL SCRIPT CAPITAL N
    ("Ntilde", "\u{00D1}"),               // LATIN CAPITAL LETTER N WITH TILDE
    ("Nu", "\u{039D}"),                   // GREEK CAPITAL LETTER NU
    ("OElig", "\u{0152}"),                // LATIN CAPITAL LIGATURE OE
    ("Oacute", "\u{00D3}"),               // LATIN CAPITAL LETTER O WITH ACUTE
    ("Ocirc", "\u{00D4}"),                // LATIN CAPITAL LETTER O WITH CIRCUMFLEX
    ("Ocy", "\u{041E}"),                  // CYRILLIC CAPITAL LETTER O
    ("Odblac", "\u{0150}"),               // LATIN CAPITAL LETTER O WITH DOUBLE ACUTE
    ("Ofr", "\u{1D512}"),                 // MATHEMATICAL FRAKTUR CAPITAL O
    ("Ograve", "\u{00D2}"),               // LATIN CAPITAL LETTER O WITH GRAVE
    ("Omacr", "\u{014C}"),                // LATIN CAPITAL LETTER O WITH MACRON
    ("Omega", "\u{03A9}"),                // GREEK CAPITAL LETTER OMEGA
    ("Omicron", "\u{039F}"),              // GREEK CAPITAL LETTER OMICRON
    ("Oopf", "\u{1D546}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL O
    ("Or", "\u{2A54}"),                   // DOUBLE LOGICAL OR
    ("Oscr", "\u{1D4AA}"),                // MATHEMATICAL SCRIPT CAPITAL O
    ("Oslash", "\u{00D8}"),               // LATIN CAPITAL LETTER O WITH STROKE
    ("Otilde", "\u{00D5}"),               // LATIN CAPITAL LETTER O WITH TILDE
    ("Otimes", "\u{2A37}"),               // MULTIPLICATION SIGN IN DOUBLE CIRCLE
    ("Ouml", "\u{00D6}"),                 // LATIN CAPITAL LETTER O WITH DIAERESIS
    ("OverBrace", "\u{23DE}"),            // TOP CURLY BRACKET
    ("OverParenthesis", "\u{23DC}"),      // TOP PARENTHESIS
    ("Pcy", "\u{041F}"),                  // CYRILLIC CAPITAL LETTER PE
    ("Pfr", "\u{1D513}"),                 // MATHEMATICAL FRAKTUR CAPITAL P
    ("Phi", "\u{03A6}"),                  // GREEK CAPITAL LETTER PHI
    ("Pi", "\u{03A0}"),                   // GREEK CAPITAL LETTER PI
    ("Popf", "\u{2119}"),                 // DOUBLE-STRUCK CAPITAL P
    ("Pr", "\u{2ABB}"),                   // DOUBLE PRECEDES
    ("Prime", "\u{2033}"),                // DOUBLE PRIME
    ("Pscr", "\u{1D4AB}"),                // MATHEMATICAL SCRIPT CAPITAL P
    ("Psi", "\u{03A8}"),                  // GREEK CAPITAL LETTER PSI
    ("QUOT", "\u{0022}"),                 // QUOTATION MARK
    ("Qfr", "\u{1D514}"),                 // MATHEMATICAL FRAKTUR CAPITAL Q
    ("Qscr", "\u{1D4AC}"),                // MATHEMATICAL SCRIPT CAPITAL Q
    ("RBarr", "\u{2910}"),                // RIGHTWARDS TWO-HEADED TRIPLE DASH ARROW
    ("Racute", "\u{0154}"),               // LATIN CAPITAL LETTER R WITH ACUTE
    ("Rang", "\u{27EB}"),                 // MATHEMATICAL RIGHT DOUBLE ANGLE BRACKET
    ("Rarr", "\u{21A0}"),                 // RIGHTWARDS TWO HEADED ARROW
    ("Rarrtl", "\u{2916}"),               // RIGHTWARDS TWO-HEADED ARROW WITH TAIL
    ("Rcaron", "\u{0158}"),               // LATIN CAPITAL LETTER R WITH CARON
    ("Rcedil", "\u{0156}"),               // LATIN CAPITAL LETTER R WITH CEDILLA
    ("Rcy", "\u{0420}"),                  // CYRILLIC CAPITAL LETTER ER
    ("Rho", "\u{03A1}"),                  // GREEK CAPITAL LETTER RHO
    ("RightDownTeeVector", "\u{295D}"),   // DOWNWARDS HARPOON WITH BARB RIGHT FROM BAR
    ("RightDownVectorBar", "\u{2955}"),   // DOWNWARDS HARPOON WITH BARB RIGHT TO BAR
    ("RightTeeVector", "\u{295B}"),       // RIGHTWARDS HARPOON WITH BARB UP FROM BAR
    ("RightTriangleBar", "\u{29D0}"),     // VERTICAL BAR BESIDE RIGHT TRIANGLE
    ("RightUpDownVector", "\u{294F}"),    // UP BARB RIGHT DOWN BARB RIGHT HARPOON
    ("RightUpTeeVector", "\u{295C}"),     // UPWARDS HARPOON WITH BARB RIGHT FROM BAR
    ("RightUpVectorBar", "\u{2954}"),     // UPWARDS HARPOON WITH BARB RIGHT TO BAR
    ("RightVectorBar", "\u{2953}"),       // RIGHTWARDS HARPOON WITH BARB UP TO BAR
    ("RoundImplies", "\u{2970}"),         // RIGHT DOUBLE ARROW WITH ROUNDED HEAD
    ("Rscr", "\u{211B}"),                 // SCRIPT CAPITAL R
    ("RuleDelayed", "\u{29F4}"),          // RULE-DELAYED
    ("SHCHcy", "\u{0429}"),               // CYRILLIC CAPITAL LETTER SHCHA
    ("SHcy", "\u{0428}"),                 // CYRILLIC CAPITAL LETTER SHA
    ("SOFTcy", "\u{042C}"),               // CYRILLIC CAPITAL LETTER SOFT SIGN
    ("Sacute", "\u{015A}"),               // LATIN CAPITAL LETTER S WITH ACUTE
    ("Sc", "\u{2ABC}"),                   // DOUBLE SUCCEEDS
    ("Scaron", "\u{0160}"),               // LATIN CAPITAL LETTER S WITH CARON
    ("Scedil", "\u{015E}"),               // LATIN CAPITAL LETTER S WITH CEDILLA
    ("Scirc", "\u{015C}"),                // LATIN CAPITAL LETTER S WITH CIRCUMFLEX
    ("Scy", "\u{0421}"),                  // CYRILLIC CAPITAL LETTER ES
    ("Sfr", "\u{1D516}"),                 // MATHEMATICAL FRAKTUR CAPITAL S
    ("Sigma", "\u{03A3}"),                // GREEK CAPITAL LETTER SIGMA
    ("Sopf", "\u{1D54A}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL S
    ("Sscr", "\u{1D4AE}"),                // MATHEMATICAL SCRIPT CAPITAL S
    ("Su", "\u{22D0}"),                   // DOUBLE SUBSET
    ("Sup", "\u{22D1}"),                  // DOUBLE SUPERSET
    ("THORN", "\u{00DE}"),                // LATIN CAPITAL LETTER THORN
    ("TSHcy", "\u{040B}"),                // CYRILLIC CAPITAL LETTER TSHE
    ("TScy", "\u{0426}"),                 // CYRILLIC CAPITAL LETTER TSE
    ("Ta", "\u{0009}"),                   // CHARACTER TABULATION
    ("Tau", "\u{03A4}"),                  // GREEK CAPITAL LETTER TAU
    ("Tcaron", "\u{0164}"),               // LATIN CAPITAL LETTER T WITH CARON
    ("Tcedil", "\u{0162}"),               // LATIN CAPITAL LETTER T WITH CEDILLA
    ("Tcy", "\u{0422}"),                  // CYRILLIC CAPITAL LETTER TE
    ("Tfr", "\u{1D517}"),                 // MATHEMATICAL FRAKTUR CAPITAL T
    ("Theta", "\u{0398}"),                // GREEK CAPITAL LETTER THETA
    ("Topf", "\u{1D54B}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL T
    ("Tscr", "\u{1D4AF}"),                // MATHEMATICAL SCRIPT CAPITAL T
    ("Tstrok", "\u{0166}"),               // LATIN CAPITAL LETTER T WITH STROKE
    ("Uacute", "\u{00DA}"),               // LATIN CAPITAL LETTER U WITH ACUTE
    ("Uarr", "\u{219F}"),                 // UPWARDS TWO HEADED ARROW
    ("Uarrocir", "\u{2949}"),             // UPWARDS TWO-HEADED ARROW FROM SMALL CIRCLE
    ("Ubrcy", "\u{040E}"),                // CYRILLIC CAPITAL LETTER SHORT U
    ("Ubreve", "\u{016C}"),               // LATIN CAPITAL LETTER U WITH BREVE
    ("Ucirc", "\u{00DB}"),                // LATIN CAPITAL LETTER U WITH CIRCUMFLEX
    ("Ucy", "\u{0423}"),                  // CYRILLIC CAPITAL LETTER U
    ("Udblac", "\u{0170}"),               // LATIN CAPITAL LETTER U WITH DOUBLE ACUTE
    ("Ufr", "\u{1D518}"),                 // MATHEMATICAL FRAKTUR CAPITAL U
    ("Ugrave", "\u{00D9}"),               // LATIN CAPITAL LETTER U WITH GRAVE
    ("Umacr", "\u{016A}"),                // LATIN CAPITAL LETTER U WITH MACRON
    ("UnderBar", "\u{0332}"),             // COMBINING LOW LINE
    ("UnderBrace", "\u{23DF}"),           // BOTTOM CURLY BRACKET
    ("UnderParenthesis", "\u{23DD}"),     // BOTTOM PARENTHESIS
    ("Uogon", "\u{0172}"),                // LATIN CAPITAL LETTER U WITH OGONEK
    ("Uopf", "\u{1D54C}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL U
    ("UpArrowBar", "\u{2912}"),           // UPWARDS ARROW TO BAR
    ("UpTeeArrow", "\u{21A5}"),           // UPWARDS ARROW FROM BAR
    ("Upsi", "\u{03D2}"),                 // GREEK UPSILON WITH HOOK SYMBOL
    ("Upsilon", "\u{03A5}"),              // GREEK CAPITAL LETTER UPSILON
    ("Uring", "\u{016E}"),                // LATIN CAPITAL LETTER U WITH RING ABOVE
    ("Uscr", "\u{1D4B0}"),                // MATHEMATICAL SCRIPT CAPITAL U
    ("Utilde", "\u{0168}"),               // LATIN CAPITAL LETTER U WITH TILDE
    ("Uuml", "\u{00DC}"),                 // LATIN CAPITAL LETTER U WITH DIAERESIS
    ("VDash", "\u{22AB}"),                // DOUBLE VERTICAL BAR DOUBLE RIGHT TURNSTILE
    ("Vbar", "\u{2AEB}"),                 // DOUBLE UP TACK
    ("Vcy", "\u{0412}"),                  // CYRILLIC CAPITAL LETTER VE
    ("Vdash", "\u{22A9}"),                // FORCES
    ("Vdashl", "\u{2AE6}"),               // LONG DASH FROM LEFT MEMBER OF DOUBLE VERTICAL
    ("Verbar", "\u{2016}"),               // DOUBLE VERTICAL LINE
    ("VerticalSeparator", "\u{2758}"),    // LIGHT VERTICAL BAR
    ("Vfr", "\u{1D519}"),                 // MATHEMATICAL FRAKTUR CAPITAL V
    ("Vopf", "\u{1D54D}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL V
    ("Vscr", "\u{1D4B1}"),                // MATHEMATICAL SCRIPT CAPITAL V
    ("Vvdash", "\u{22AA}"),               // TRIPLE VERTICAL BAR RIGHT TURNSTILE
    ("Wcirc", "\u{0174}"),                // LATIN CAPITAL LETTER W WITH CIRCUMFLEX
    ("Wfr", "\u{1D51A}"),                 // MATHEMATICAL FRAKTUR CAPITAL W
    ("Wopf", "\u{1D54E}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL W
    ("Wscr", "\u{1D4B2}"),                // MATHEMATICAL SCRIPT CAPITAL W
    ("Xfr", "\u{1D51B}"),                 // MATHEMATICAL FRAKTUR CAPITAL X
    ("Xi", "\u{039E}"),                   // GREEK CAPITAL LETTER XI
    ("Xopf", "\u{1D54F}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL X
    ("Xscr", "\u{1D4B3}"),                // MATHEMATICAL SCRIPT CAPITAL X
    ("YAcy", "\u{042F}"),                 // CYRILLIC CAPITAL LETTER YA
    ("YIcy", "\u{0407}"),                 // CYRILLIC CAPITAL LETTER YI
    ("YUcy", "\u{042E}"),                 // CYRILLIC CAPITAL LETTER YU
    ("Yacute", "\u{00DD}"),               // LATIN CAPITAL LETTER Y WITH ACUTE
    ("Ycirc", "\u{0176}"),                // LATIN CAPITAL LETTER Y WITH CIRCUMFLEX
    ("Ycy", "\u{042B}"),                  // CYRILLIC CAPITAL LETTER YERU
    ("Yfr", "\u{1D51C}"),                 // MATHEMATICAL FRAKTUR CAPITAL Y
    ("Yopf", "\u{1D550}"),                // MATHEMATICAL DOUBLE-STRUCK CAPITAL Y
    ("Yscr", "\u{1D4B4}"),                // MATHEMATICAL SCRIPT CAPITAL Y
    ("Yuml", "\u{0178}"),                 // LATIN CAPITAL LETTER Y WITH DIAERESIS
    ("ZHcy", "\u{0416}"),                 // CYRILLIC CAPITAL LETTER ZHE
    ("Zacute", "\u{0179}"),               // LATIN CAPITAL LETTER Z WITH ACUTE
    ("Zcaron", "\u{017D}"),               // LATIN CAPITAL LETTER Z WITH CARON
    ("Zcy", "\u{0417}"),                  // CYRILLIC CAPITAL LETTER ZE
    ("Zdot", "\u{017B}"),                 // LATIN CAPITAL LETTER Z WITH DOT ABOVE
    ("ZeroWidthSpace", "\u{200B}"),       // ZERO WIDTH SPACE
    ("Zeta", "\u{0396}"),                 // GREEK CAPITAL LETTER ZETA
    ("Zfr", "\u{2128}"),                  // BLACK-LETTER CAPITAL Z
    ("Zscr", "\u{1D4B5}"),                // MATHEMATICAL SCRIPT CAPITAL Z
    ("aacute", "\u{00E1}"),               // LATIN SMALL LETTER A WITH ACUTE
    ("abreve", "\u{0103}"),               // LATIN SMALL LETTER A WITH BREVE
    ("ac", "\u{223E}"),                   // INVERTED LAZY S
    ("acd", "\u{223F}"),                  // SINE WAVE
    ("acirc", "\u{00E2}"),                // LATIN SMALL LETTER A WITH CIRCUMFLEX
    ("acute", "\u{00B4}"),                // ACUTE ACCENT
    ("acy", "\u{0430}"),                  // CYRILLIC SMALL LETTER A
    ("aelig", "\u{00E6}"),                // LATIN SMALL LETTER AE
    ("afr", "\u{1D51E}"),                 // MATHEMATICAL FRAKTUR SMALL A
    ("agrave", "\u{00E0}"),               // LATIN SMALL LETTER A WITH GRAVE
    ("alefsym", "\u{2135}"),              // ALEF SYMBOL
    ("alpha", "\u{03B1}"),                // GREEK SMALL LETTER ALPHA
    ("amacr", "\u{0101}"),                // LATIN SMALL LETTER A WITH MACRON
    ("amalg", "\u{2A3F}"),                // AMALGAMATION OR COPRODUCT
    ("amp", "\u{0026}"),                  // AMPERSAND
    ("and", "\u{2227}"),                  // LOGICAL AND
    ("andand", "\u{2A55}"),               // TWO INTERSECTING LOGICAL AND
    ("andd", "\u{2A5C}"),                 // LOGICAL AND WITH HORIZONTAL DASH
    ("andslope", "\u{2A58}"),             // SLOPING LARGE AND
    ("andv", "\u{2A5A}"),                 // LOGICAL AND WITH MIDDLE STEM
    ("ang", "\u{2220}"),                  // ANGLE
    ("ange", "\u{29A4}"),                 // ANGLE WITH UNDERBAR
    ("angmsd", "\u{2221}"),               // MEASURED ANGLE
    ("angmsdaa", "\u{29A8}"), /* MEASURED ANGLE WITH OPEN ARM ENDING IN ARROW POINTING UP AND RIGHT */
    ("angmsda", "\u{29A9}"),  // MEASURED ANGLE WITH OPEN ARM ENDING IN ARROW POINTING UP AND LEFT
    ("angmsdac", "\u{29AA}"), /* MEASURED ANGLE WITH OPEN ARM ENDING IN ARROW POINTING DOWN AND RIGHT */
    ("angmsdad", "\u{29AB}"), /* MEASURED ANGLE WITH OPEN ARM ENDING IN ARROW POINTING DOWN AND LEFT */
    ("angmsdae", "\u{29AC}"), /* MEASURED ANGLE WITH OPEN ARM ENDING IN ARROW POINTING RIGHT AND UP */
    ("angmsdaf", "\u{29AD}"), // MEASURED ANGLE WITH OPEN ARM ENDING IN ARROW POINTING LEFT AND UP
    ("angmsdag", "\u{29AE}"), /* MEASURED ANGLE WITH OPEN ARM ENDING IN ARROW POINTING RIGHT AND DOWN */
    ("angmsdah", "\u{29AF}"), /* MEASURED ANGLE WITH OPEN ARM ENDING IN ARROW POINTING LEFT AND DOWN */
    ("angrt", "\u{221F}"),    // RIGHT ANGLE
    ("angrtv", "\u{22BE}"),   // RIGHT ANGLE WITH ARC
    ("angrtvbd", "\u{299D}"), // MEASURED RIGHT ANGLE WITH DOT
    ("angsph", "\u{2222}"),   // SPHERICAL ANGLE
    ("angst", "\u{212B}"),    // ANGSTROM SIGN
    ("angzarr", "\u{237C}"),  // RIGHT ANGLE WITH DOWNWARDS ZIGZAG ARROW
    ("aogon", "\u{0105}"),    // LATIN SMALL LETTER A WITH OGONEK
    ("aopf", "\u{1D552}"),    // MATHEMATICAL DOUBLE-STRUCK SMALL A
    ("apE", "\u{2A70}"),      // APPROXIMATELY EQUAL OR EQUAL TO
    ("apacir", "\u{2A6F}"),   // ALMOST EQUAL TO WITH CIRCUMFLEX ACCENT
    ("ape", "\u{224A}"),      // ALMOST EQUAL OR EQUAL TO
    ("apid", "\u{224B}"),     // TRIPLE TILDE
    ("apos", "\u{0027}"),     // APOSTROPHE
    ("aring", "\u{00E5}"),    // LATIN SMALL LETTER A WITH RING ABOVE
    ("ascr", "\u{1D4B6}"),    // MATHEMATICAL SCRIPT SMALL A
    ("ast", "\u{002A}"),      // ASTERISK
    ("asymp", "\u{2248}"),    // ALMOST EQUAL TO
    ("asympeq", "\u{224D}"),  // EQUIVALENT TO
    ("atilde", "\u{00E3}"),   // LATIN SMALL LETTER A WITH TILDE
    ("auml", "\u{00E4}"),     // LATIN SMALL LETTER A WITH DIAERESIS
    ("awconint", "\u{2233}"), // ANTICLOCKWISE CONTOUR INTEGRAL
    ("awint", "\u{2A11}"),    // ANTICLOCKWISE INTEGRATION
    ("bNot", "\u{2AED}"),     // REVERSED DOUBLE STROKE NOT SIGN
    ("barvee", "\u{22BD}"),   // NOR
    ("barwed", "\u{2305}"),   // PROJECTIVE
    ("bbrk", "\u{23B5}"),     // BOTTOM SQUARE BRACKET
    ("bbrktbrk", "\u{23B6}"), // BOTTOM SQUARE BRACKET OVER TOP SQUARE BRACKET
    ("bcong", "\u{224C}"),    // ALL EQUAL TO
    ("bcy", "\u{0431}"),      // CYRILLIC SMALL LETTER BE
    ("becaus", "\u{2235}"),   // BECAUSE
    ("bemptyv", "\u{29B0}"),  // REVERSED EMPTY SET
    ("bepsi", "\u{03F6}"),    // GREEK REVERSED LUNATE EPSILON SYMBOL
    ("bernou", "\u{212C}"),   // SCRIPT CAPITAL B
    ("beta", "\u{03B2}"),     // GREEK SMALL LETTER BETA
    ("beth", "\u{2136}"),     // BET SYMBOL
    ("bfr", "\u{1D51F}"),     // MATHEMATICAL FRAKTUR SMALL B
    ("blank", "\u{2423}"),    // OPEN BOX
    ("blk12", "\u{2592}"),    // MEDIUM SHADE
    ("blk14", "\u{2591}"),    // LIGHT SHADE
    ("blk34", "\u{2593}"),    // DARK SHADE
    ("block", "\u{2588}"),    // FULL BLOCK
    ("bnot", "\u{2310}"),     // REVERSED NOT SIGN
    ("bopf", "\u{1D553}"),    // MATHEMATICAL DOUBLE-STRUCK SMALL B
    ("bottom", "\u{22A5}"),   // UP TACK
    ("bowtie", "\u{22C8}"),   // BOWTIE
    ("boxDL", "\u{2557}"),    // BOX DRAWINGS DOUBLE DOWN AND LEFT
    ("boxDR", "\u{2554}"),    // BOX DRAWINGS DOUBLE DOWN AND RIGHT
    ("boxDl", "\u{2556}"),    // BOX DRAWINGS DOWN DOUBLE AND LEFT SINGLE
    ("boxDr", "\u{2553}"),    // BOX DRAWINGS DOWN DOUBLE AND RIGHT SINGLE
    ("boxH", "\u{2550}"),     // BOX DRAWINGS DOUBLE HORIZONTAL
    ("boxHD", "\u{2566}"),    // BOX DRAWINGS DOUBLE DOWN AND HORIZONTAL
    ("boxHU", "\u{2569}"),    // BOX DRAWINGS DOUBLE UP AND HORIZONTAL
    ("boxHd", "\u{2564}"),    // BOX DRAWINGS DOWN SINGLE AND HORIZONTAL DOUBLE
    ("boxHu", "\u{2567}"),    // BOX DRAWINGS UP SINGLE AND HORIZONTAL DOUBLE
    ("boxUL", "\u{255D}"),    // BOX DRAWINGS DOUBLE UP AND LEFT
    ("boxUR", "\u{255A}"),    // BOX DRAWINGS DOUBLE UP AND RIGHT
    ("boxUl", "\u{255C}"),    // BOX DRAWINGS UP DOUBLE AND LEFT SINGLE
    ("boxUr", "\u{2559}"),    // BOX DRAWINGS UP DOUBLE AND RIGHT SINGLE
    ("boxV", "\u{2551}"),     // BOX DRAWINGS DOUBLE VERTICAL
    ("boxVH", "\u{256C}"),    // BOX DRAWINGS DOUBLE VERTICAL AND HORIZONTAL
    ("boxVL", "\u{2563}"),    // BOX DRAWINGS DOUBLE VERTICAL AND LEFT
    ("boxVR", "\u{2560}"),    // BOX DRAWINGS DOUBLE VERTICAL AND RIGHT
    ("boxVh", "\u{256B}"),    // BOX DRAWINGS VERTICAL DOUBLE AND HORIZONTAL SINGLE
    ("boxVl", "\u{2562}"),    // BOX DRAWINGS VERTICAL DOUBLE AND LEFT SINGLE
    ("boxVr", "\u{255F}"),    // BOX DRAWINGS VERTICAL DOUBLE AND RIGHT SINGLE
    ("boxbox", "\u{29C9}"),   // TWO JOINED SQUARES
    ("boxdL", "\u{2555}"),    // BOX DRAWINGS DOWN SINGLE AND LEFT DOUBLE
    ("boxdR", "\u{2552}"),    // BOX DRAWINGS DOWN SINGLE AND RIGHT DOUBLE
    ("boxdl", "\u{2510}"),    // BOX DRAWINGS LIGHT DOWN AND LEFT
    ("boxdr", "\u{250C}"),    // BOX DRAWINGS LIGHT DOWN AND RIGHT
    ("boxh", "\u{2500}"),     // BOX DRAWINGS LIGHT HORIZONTAL
    ("boxhD", "\u{2565}"),    // BOX DRAWINGS DOWN DOUBLE AND HORIZONTAL SINGLE
    ("boxhU", "\u{2568}"),    // BOX DRAWINGS UP DOUBLE AND HORIZONTAL SINGLE
    ("boxhd", "\u{252C}"),    // BOX DRAWINGS LIGHT DOWN AND HORIZONTAL
    ("boxhu", "\u{2534}"),    // BOX DRAWINGS LIGHT UP AND HORIZONTAL
    ("boxuL", "\u{255B}"),    // BOX DRAWINGS UP SINGLE AND LEFT DOUBLE
    ("boxuR", "\u{2558}"),    // BOX DRAWINGS UP SINGLE AND RIGHT DOUBLE
    ("boxul", "\u{2518}"),    // BOX DRAWINGS LIGHT UP AND LEFT
    ("boxur", "\u{2514}"),    // BOX DRAWINGS LIGHT UP AND RIGHT
    ("boxv", "\u{2502}"),     // BOX DRAWINGS LIGHT VERTICAL
    ("boxvH", "\u{256A}"),    // BOX DRAWINGS VERTICAL SINGLE AND HORIZONTAL DOUBLE
    ("boxvL", "\u{2561}"),    // BOX DRAWINGS VERTICAL SINGLE AND LEFT DOUBLE
    ("boxvR", "\u{255E}"),    // BOX DRAWINGS VERTICAL SINGLE AND RIGHT DOUBLE
    ("boxvh", "\u{253C}"),    // BOX DRAWINGS LIGHT VERTICAL AND HORIZONTAL
    ("boxvl", "\u{2524}"),    // BOX DRAWINGS LIGHT VERTICAL AND LEFT
    ("boxvr", "\u{251C}"),    // BOX DRAWINGS LIGHT VERTICAL AND RIGHT
    ("bprime", "\u{2035}"),   // REVERSED PRIME
    ("breve", "\u{02D8}"),    // BREVE
    ("brvbar", "\u{00A6}"),   // BROKEN BAR
    ("bscr", "\u{1D4B7}"),    // MATHEMATICAL SCRIPT SMALL B
    ("bsemi", "\u{204F}"),    // REVERSED SEMICOLON
    ("bsim", "\u{223D}"),     // REVERSED TILDE
    ("bsime", "\u{22CD}"),    // REVERSED TILDE EQUALS
    ("bsol", "\u{005C}"),     // REVERSE SOLIDUS
    ("bsol", "\u{29C5}"),     // SQUARED FALLING DIAGONAL SLASH
    ("bull", "\u{2022}"),     // BULLET
    ("bump", "\u{224E}"),     // GEOMETRICALLY EQUIVALENT TO
    ("bumpE", "\u{2AAE}"),    // EQUALS SIGN WITH BUMPY ABOVE
    ("bumpe", "\u{224F}"),    // DIFFERENCE BETWEEN
    ("cacute", "\u{0107}"),   // LATIN SMALL LETTER C WITH ACUTE
    ("cap", "\u{2229}"),      // INTERSECTION
    ("capand", "\u{2A44}"),   // INTERSECTION WITH LOGICAL AND
    ("capbrcup", "\u{2A49}"), // INTERSECTION ABOVE BAR ABOVE UNION
    ("capcap", "\u{2A4B}"),   // INTERSECTION BESIDE AND JOINED WITH INTERSECTION
    ("capcup", "\u{2A47}"),   // INTERSECTION ABOVE UNION
    ("capdot", "\u{2A40}"),   // INTERSECTION WITH DOT
    ("caret", "\u{2041}"),    // CARET INSERTION POINT
    ("caron", "\u{02C7}"),    // CARON
    ("ccaps", "\u{2A4D}"),    // CLOSED INTERSECTION WITH SERIFS
    ("ccaron", "\u{010D}"),   // LATIN SMALL LETTER C WITH CARON
    ("ccedil", "\u{00E7}"),   // LATIN SMALL LETTER C WITH CEDILLA
    ("ccirc", "\u{0109}"),    // LATIN SMALL LETTER C WITH CIRCUMFLEX
    ("ccups", "\u{2A4C}"),    // CLOSED UNION WITH SERIFS
    ("ccupssm", "\u{2A50}"),  // CLOSED UNION WITH SERIFS AND SMASH PRODUCT
    ("cdot", "\u{010B}"),     // LATIN SMALL LETTER C WITH DOT ABOVE
    ("cedil", "\u{00B8}"),    // CEDILLA
    ("cemptyv", "\u{29B2}"),  // EMPTY SET WITH SMALL CIRCLE ABOVE
    ("cent", "\u{00A2}"),     // CENT SIGN
    ("cfr", "\u{1D520}"),     // MATHEMATICAL FRAKTUR SMALL C
    ("chcy", "\u{0447}"),     // CYRILLIC SMALL LETTER CHE
    ("check", "\u{2713}"),    // CHECK MARK
    ("chi", "\u{03C7}"),      // GREEK SMALL LETTER CHI
    ("cir", "\u{25CB}"),      // WHITE CIRCLE
    ("cirE", "\u{29C3}"),     // CIRCLE WITH TWO HORIZONTAL STROKES TO THE RIGHT
    ("circ", "\u{02C6}"),     // MODIFIER LETTER CIRCUMFLEX ACCENT
    ("cire", "\u{2257}"),     // RING EQUAL TO
    ("cirfnint", "\u{2A10}"), // CIRCULATION FUNCTION
    ("cirmid", "\u{2AEF}"),   // VERTICAL LINE WITH CIRCLE ABOVE
    ("cirscir", "\u{29C2}"),  // CIRCLE WITH SMALL CIRCLE TO THE RIGHT
    ("clubs", "\u{2663}"),    // BLACK CLUB SUIT
    ("colon", "\u{003A}"),    // COLON
    ("colone", "\u{2254}"),   // COLON EQUALS
    ("comma", "\u{002C}"),    // COMMA
    ("commat", "\u{0040}"),   // COMMERCIAL AT
    ("comp", "\u{2201}"),     // COMPLEMENT
    ("compfn", "\u{2218}"),   // RING OPERATOR
    ("cong", "\u{2245}"),     // APPROXIMATELY EQUAL TO
    ("congdot", "\u{2A6D}"),  // CONGRUENT WITH DOT ABOVE
    ("conint", "\u{222E}"),   // CONTOUR INTEGRAL
    ("copf", "\u{1D554}"),    // MATHEMATICAL DOUBLE-STRUCK SMALL C
    ("coprod", "\u{2210}"),   // N-ARY COPRODUCT
    ("copy", "\u{00A9}"),     // COPYRIGHT SIGN
    ("copysr", "\u{2117}"),   // SOUND RECORDING COPYRIGHT
    ("crarr", "\u{21B5}"),    // DOWNWARDS ARROW WITH CORNER LEFTWARDS
    ("cross", "\u{2717}"),    // BALLOT X
    ("cscr", "\u{1D4B8}"),    // MATHEMATICAL SCRIPT SMALL C
    ("csu", "\u{2ACF}"),      // CLOSED SUBSET
    ("csube", "\u{2AD1}"),    // CLOSED SUBSET OR EQUAL TO
    ("csup", "\u{2AD0}"),     // CLOSED SUPERSET
    ("csupe", "\u{2AD2}"),    // CLOSED SUPERSET OR EQUAL TO
    ("ctdot", "\u{22EF}"),    // MIDLINE HORIZONTAL ELLIPSIS
    ("cudarrl", "\u{2938}"),  // RIGHT-SIDE ARC CLOCKWISE ARROW
    ("cudarrr", "\u{2935}"),  // ARROW POINTING RIGHTWARDS THEN CURVING DOWNWARDS
    ("cuepr", "\u{22DE}"),    // EQUAL TO OR PRECEDES
    ("cuesc", "\u{22DF}"),    // EQUAL TO OR SUCCEEDS
    ("cularr", "\u{21B6}"),   // ANTICLOCKWISE TOP SEMICIRCLE ARROW
    ("cularrp", "\u{293D}"),  // TOP ARC ANTICLOCKWISE ARROW WITH PLUS
    ("cup", "\u{222A}"),      // UNION
    ("cupbrcap", "\u{2A48}"), // UNION ABOVE BAR ABOVE INTERSECTION
    ("cupcap", "\u{2A46}"),   // UNION ABOVE INTERSECTION
    ("cupcup", "\u{2A4A}"),   // UNION BESIDE AND JOINED WITH UNION
    ("cupdot", "\u{228D}"),   // MULTISET MULTIPLICATION
    ("cupor", "\u{2A45}"),    // UNION WITH LOGICAL OR
    ("curarr", "\u{21B7}"),   // CLOCKWISE TOP SEMICIRCLE ARROW
    ("curarrm", "\u{293C}"),  // TOP ARC CLOCKWISE ARROW WITH MINUS
    ("curren", "\u{00A4}"),   // CURRENCY SIGN
    ("cuvee", "\u{22CE}"),    // CURLY LOGICAL OR
    ("cuwed", "\u{22CF}"),    // CURLY LOGICAL AND
    ("cwconint", "\u{2232}"), // CLOCKWISE CONTOUR INTEGRAL
    ("cwint", "\u{2231}"),    // CLOCKWISE INTEGRAL
    ("cylcty", "\u{232D}"),   // CYLINDRICITY
    ("dArr", "\u{21D3}"),     // DOWNWARDS DOUBLE ARROW
    ("dHar", "\u{2965}"), /* DOWNWARDS HARPOON WITH BARB LEFT BESIDE DOWNWARDS HARPOON WITH BARB RIGHT */
    ("dagger", "\u{2020}"), // DAGGER
    ("daleth", "\u{2138}"), // DALET SYMBOL
    ("darr", "\u{2193}"), // DOWNWARDS ARROW
    ("dashv", "\u{22A3}"), // LEFT TACK
    ("dblac", "\u{02DD}"), // DOUBLE ACUTE ACCENT
    ("dcaron", "\u{010F}"), // LATIN SMALL LETTER D WITH CARON
    ("dcy", "\u{0434}"),  // CYRILLIC SMALL LETTER DE
    ("ddarr", "\u{21CA}"), // DOWNWARDS PAIRED ARROWS
    ("deg", "\u{00B0}"),  // DEGREE SIGN
    ("delta", "\u{03B4}"), // GREEK SMALL LETTER DELTA
    ("demptyv", "\u{29B1}"), // EMPTY SET WITH OVERBAR
    ("dfisht", "\u{297F}"), // DOWN FISH TAIL
    ("dfr", "\u{1D521}"), // MATHEMATICAL FRAKTUR SMALL D
    ("dharl", "\u{21C3}"), // DOWNWARDS HARPOON WITH BARB LEFTWARDS
    ("dharr", "\u{21C2}"), // DOWNWARDS HARPOON WITH BARB RIGHTWARDS
    ("diam", "\u{22C4}"), // DIAMOND OPERATOR
    ("diams", "\u{2666}"), // BLACK DIAMOND SUIT
    ("disin", "\u{22F2}"), // ELEMENT OF WITH LONG HORIZONTAL STROKE
    ("divide", "\u{00F7}"), // DIVISION SIGN
    ("divonx", "\u{22C7}"), // DIVISION TIMES
    ("djcy", "\u{0452}"), // CYRILLIC SMALL LETTER DJE
    ("dlcorn", "\u{231E}"), // BOTTOM LEFT CORNER
    ("dlcrop", "\u{230D}"), // BOTTOM LEFT CROP
    ("dollar", "\u{0024}"), // DOLLAR SIGN
    ("dopf", "\u{1D555}"), // MATHEMATICAL DOUBLE-STRUCK SMALL D
    ("dot", "\u{02D9}"),  // DOT ABOVE
    ("drcorn", "\u{231F}"), // BOTTOM RIGHT CORNER
    ("drcrop", "\u{230C}"), // BOTTOM RIGHT CROP
    ("dscr", "\u{1D4B9}"), // MATHEMATICAL SCRIPT SMALL D
    ("dscy", "\u{0455}"), // CYRILLIC SMALL LETTER DZE
    ("dsol", "\u{29F6}"), // SOLIDUS WITH OVERBAR
    ("dstrok", "\u{0111}"), // LATIN SMALL LETTER D WITH STROKE
    ("dtdot", "\u{22F1}"), // DOWN RIGHT DIAGONAL ELLIPSIS
    ("dtri", "\u{25BF}"), // WHITE DOWN-POINTING SMALL TRIANGLE
    ("dtrif", "\u{25BE}"), // BLACK DOWN-POINTING SMALL TRIANGLE
    ("duarr", "\u{21F5}"), // DOWNWARDS ARROW LEFTWARDS OF UPWARDS ARROW
    ("duhar", "\u{296F}"), /* DOWNWARDS HARPOON WITH BARB LEFT BESIDE UPWARDS HARPOON WITH BARB RIGHT */
    ("dwangle", "\u{29A6}"), // OBLIQUE ANGLE OPENING UP
    ("dzcy", "\u{045F}"),  // CYRILLIC SMALL LETTER DZHE
    ("dzigrarr", "\u{27FF}"), // LONG RIGHTWARDS SQUIGGLE ARROW
    ("eDDot", "\u{2A77}"), // EQUALS SIGN WITH TWO DOTS ABOVE AND TWO DOTS BELOW
    ("eDot", "\u{2251}"),  // GEOMETRICALLY EQUAL TO
    ("eacute", "\u{00E9}"), // LATIN SMALL LETTER E WITH ACUTE
    ("easter", "\u{2A6E}"), // EQUALS WITH ASTERISK
    ("ecaron", "\u{011B}"), // LATIN SMALL LETTER E WITH CARON
    ("ecir", "\u{2256}"),  // RING IN EQUAL TO
    ("ecirc", "\u{00EA}"), // LATIN SMALL LETTER E WITH CIRCUMFLEX
    ("ecolon", "\u{2255}"), // EQUALS COLON
    ("ecy", "\u{044D}"),   // CYRILLIC SMALL LETTER E
    ("edot", "\u{0117}"),  // LATIN SMALL LETTER E WITH DOT ABOVE
    ("efDot", "\u{2252}"), // APPROXIMATELY EQUAL TO OR THE IMAGE OF
    ("efr", "\u{1D522}"),  // MATHEMATICAL FRAKTUR SMALL E
    ("eg", "\u{2A9A}"),    // DOUBLE-LINE EQUAL TO OR GREATER-THAN
    ("egrave", "\u{00E8}"), // LATIN SMALL LETTER E WITH GRAVE
    ("egs", "\u{2A96}"),   // SLANTED EQUAL TO OR GREATER-THAN
    ("egsdot", "\u{2A98}"), // SLANTED EQUAL TO OR GREATER-THAN WITH DOT INSIDE
    ("el", "\u{2A99}"),    // DOUBLE-LINE EQUAL TO OR LESS-THAN
    ("elinters", "\u{23E7}"), // ELECTRICAL INTERSECTION
    ("ell", "\u{2113}"),   // SCRIPT SMALL L
    ("els", "\u{2A95}"),   // SLANTED EQUAL TO OR LESS-THAN
    ("elsdot", "\u{2A97}"), // SLANTED EQUAL TO OR LESS-THAN WITH DOT INSIDE
    ("emacr", "\u{0113}"), // LATIN SMALL LETTER E WITH MACRON
    ("empty", "\u{2205}"), // EMPTY SET
    ("emsp", "\u{2003}"),  // EM SPACE
    ("emsp13", "\u{2004}"), // THREE-PER-EM SPACE
    ("emsp14", "\u{2005}"), // FOUR-PER-EM SPACE
    ("eng", "\u{014B}"),   // LATIN SMALL LETTER ENG
    ("ensp", "\u{2002}"),  // EN SPACE
    ("eogon", "\u{0119}"), // LATIN SMALL LETTER E WITH OGONEK
    ("eopf", "\u{1D556}"), // MATHEMATICAL DOUBLE-STRUCK SMALL E
    ("epar", "\u{22D5}"),  // EQUAL AND PARALLEL TO
    ("eparsl", "\u{29E3}"), // EQUALS SIGN AND SLANTED PARALLEL
    ("eplus", "\u{2A71}"), // EQUALS SIGN ABOVE PLUS SIGN
    ("epsi", "\u{03F5}"),  // GREEK LUNATE EPSILON SYMBOL
    ("epsiv", "\u{03B5}"), // GREEK SMALL LETTER EPSILON
    ("equals", "\u{003D}"), // EQUALS SIGN
    ("equest", "\u{225F}"), // QUESTIONED EQUAL TO
    ("equiv", "\u{2261}"), // IDENTICAL TO
    ("equivDD", "\u{2A78}"), // EQUIVALENT WITH FOUR DOTS ABOVE
    ("eqvparsl", "\u{29E5}"), // IDENTICAL TO AND SLANTED PARALLEL
    ("erDot", "\u{2253}"), // IMAGE OF OR APPROXIMATELY EQUAL TO
    ("erarr", "\u{2971}"), // EQUALS SIGN ABOVE RIGHTWARDS ARROW
    ("escr", "\u{212F}"),  // SCRIPT SMALL E
    ("esdot", "\u{2250}"), // APPROACHES THE LIMIT
    ("esim", "\u{2242}"),  // MINUS TILDE
    ("eta", "\u{03B7}"),   // GREEK SMALL LETTER ETA
    ("eth", "\u{00F0}"),   // LATIN SMALL LETTER ETH
    ("euml", "\u{00EB}"),  // LATIN SMALL LETTER E WITH DIAERESIS
    ("euro", "\u{20AC}"),  // EURO SIGN
    ("excl", "\u{0021}"),  // EXCLAMATION MARK
    ("exist", "\u{2203}"), // THERE EXISTS
    ("fcy", "\u{0444}"),   // CYRILLIC SMALL LETTER EF
    ("female", "\u{2640}"), // FEMALE SIGN
    ("ffilig", "\u{FB03}"), // LATIN SMALL LIGATURE FFI
    ("fflig", "\u{FB00}"), // LATIN SMALL LIGATURE FF
    ("ffllig", "\u{FB04}"), // LATIN SMALL LIGATURE FFL
    ("ffr", "\u{1D523}"),  // MATHEMATICAL FRAKTUR SMALL F
    ("filig", "\u{FB01}"), // LATIN SMALL LIGATURE FI
    ("flat", "\u{266D}"),  // MUSIC FLAT SIGN
    ("fllig", "\u{FB02}"), // LATIN SMALL LIGATURE FL
    ("fltns", "\u{25B1}"), // WHITE PARALLELOGRAM
    ("fnof", "\u{0192}"),  // LATIN SMALL LETTER F WITH HOOK
    ("fopf", "\u{1D557}"), // MATHEMATICAL DOUBLE-STRUCK SMALL F
    ("forall", "\u{2200}"), // FOR ALL
    ("fork", "\u{22D4}"),  // PITCHFORK
    ("forkv", "\u{2AD9}"), // ELEMENT OF OPENING DOWNWARDS
    ("fpartint", "\u{2A0D}"), // FINITE PART INTEGRAL
    ("frac12", "\u{00BD}"), // VULGAR FRACTION ONE HALF
    ("frac13", "\u{2153}"), // VULGAR FRACTION ONE THIRD
    ("frac14", "\u{00BC}"), // VULGAR FRACTION ONE QUARTER
    ("frac15", "\u{2155}"), // VULGAR FRACTION ONE FIFTH
    ("frac16", "\u{2159}"), // VULGAR FRACTION ONE SIXTH
    ("frac18", "\u{215B}"), // VULGAR FRACTION ONE EIGHTH
    ("frac23", "\u{2154}"), // VULGAR FRACTION TWO THIRDS
    ("frac25", "\u{2156}"), // VULGAR FRACTION TWO FIFTHS
    ("frac34", "\u{00BE}"), // VULGAR FRACTION THREE QUARTERS
    ("frac35", "\u{2157}"), // VULGAR FRACTION THREE FIFTHS
    ("frac38", "\u{215C}"), // VULGAR FRACTION THREE EIGHTHS
    ("frac45", "\u{2158}"), // VULGAR FRACTION FOUR FIFTHS
    ("frac56", "\u{215A}"), // VULGAR FRACTION FIVE SIXTHS
    ("frac58", "\u{215D}"), // VULGAR FRACTION FIVE EIGHTHS
    ("frac78", "\u{215E}"), // VULGAR FRACTION SEVEN EIGHTHS
    ("frasl", "\u{2044}"), // FRACTION SLASH
    ("frown", "\u{2322}"), // FROWN
    ("fscr", "\u{1D4BB}"), // MATHEMATICAL SCRIPT SMALL F
    ("gE", "\u{2267}"),    // GREATER-THAN OVER EQUAL TO
    ("gEl", "\u{2A8C}"),   // GREATER-THAN ABOVE DOUBLE-LINE EQUAL ABOVE LESS-THAN
    ("gacute", "\u{01F5}"), // LATIN SMALL LETTER G WITH ACUTE
    ("gamma", "\u{03B3}"), // GREEK SMALL LETTER GAMMA
    ("gammad", "\u{03DD}"), // GREEK SMALL LETTER DIGAMMA
    ("gap", "\u{2A86}"),   // GREATER-THAN OR APPROXIMATE
    ("gbreve", "\u{011F}"), // LATIN SMALL LETTER G WITH BREVE
    ("gcirc", "\u{011D}"), // LATIN SMALL LETTER G WITH CIRCUMFLEX
    ("gcy", "\u{0433}"),   // CYRILLIC SMALL LETTER GHE
    ("gdot", "\u{0121}"),  // LATIN SMALL LETTER G WITH DOT ABOVE
    ("ge", "\u{2265}"),    // GREATER-THAN OR EQUAL TO
    ("gel", "\u{22DB}"),   // GREATER-THAN EQUAL TO OR LESS-THAN
    ("ges", "\u{2A7E}"),   // GREATER-THAN OR SLANTED EQUAL TO
    ("gescc", "\u{2AA9}"), // GREATER-THAN CLOSED BY CURVE ABOVE SLANTED EQUAL
    ("gesdot", "\u{2A80}"), // GREATER-THAN OR SLANTED EQUAL TO WITH DOT INSIDE
    ("gesdoto", "\u{2A82}"), // GREATER-THAN OR SLANTED EQUAL TO WITH DOT ABOVE
    ("gesdotol", "\u{2A84}"), // GREATER-THAN OR SLANTED EQUAL TO WITH DOT ABOVE LEFT
    ("gesles", "\u{2A94}"), // GREATER-THAN ABOVE SLANTED EQUAL ABOVE LESS-THAN ABOVE SLANTED EQUAL
    ("gfr", "\u{1D524}"),  // MATHEMATICAL FRAKTUR SMALL G
    ("gimel", "\u{2137}"), // GIMEL SYMBOL
    ("gjcy", "\u{0453}"),  // CYRILLIC SMALL LETTER GJE
    ("gl", "\u{2277}"),    // GREATER-THAN OR LESS-THAN
    ("glE", "\u{2A92}"),   // GREATER-THAN ABOVE LESS-THAN ABOVE DOUBLE-LINE EQUAL
    ("gla", "\u{2AA5}"),   // GREATER-THAN BESIDE LESS-THAN
    ("glj", "\u{2AA4}"),   // GREATER-THAN OVERLAPPING LESS-THAN
    ("gnE", "\u{2269}"),   // GREATER-THAN BUT NOT EQUAL TO
    ("gnap", "\u{2A8A}"),  // GREATER-THAN AND NOT APPROXIMATE
    ("gne", "\u{2A88}"),   // GREATER-THAN AND SINGLE-LINE NOT EQUAL TO
    ("gnsim", "\u{22E7}"), // GREATER-THAN BUT NOT EQUIVALENT TO
    ("gopf", "\u{1D558}"), // MATHEMATICAL DOUBLE-STRUCK SMALL G
    ("grave", "\u{0060}"), // GRAVE ACCENT
    ("gscr", "\u{210A}"),  // SCRIPT SMALL G
    ("gsim", "\u{2273}"),  // GREATER-THAN OR EQUIVALENT TO
    ("gsime", "\u{2A8E}"), // GREATER-THAN ABOVE SIMILAR OR EQUAL
    ("gsiml", "\u{2A90}"), // GREATER-THAN ABOVE SIMILAR ABOVE LESS-THAN
    ("gt", "\u{003E}"),    // GREATER-THAN SIGN
    ("gtcc", "\u{2AA7}"),  // GREATER-THAN CLOSED BY CURVE
    ("gtcir", "\u{2A7A}"), // GREATER-THAN WITH CIRCLE INSIDE
    ("gtdot", "\u{22D7}"), // GREATER-THAN WITH DOT
    ("gtlPar", "\u{2995}"), // DOUBLE LEFT ARC GREATER-THAN BRACKET
    ("gtquest", "\u{2A7C}"), // GREATER-THAN WITH QUESTION MARK ABOVE
    ("gtrarr", "\u{2978}"), // GREATER-THAN ABOVE RIGHTWARDS ARROW
    ("hArr", "\u{21D4}"),  // LEFT RIGHT DOUBLE ARROW
    ("hairsp", "\u{200A}"), // HAIR SPACE
    ("hamilt", "\u{210B}"), // SCRIPT CAPITAL H
    ("hardcy", "\u{044A}"), // CYRILLIC SMALL LETTER HARD SIGN
    ("harr", "\u{2194}"),  // LEFT RIGHT ARROW
    ("harrcir", "\u{2948}"), // LEFT RIGHT ARROW THROUGH SMALL CIRCLE
    ("harrw", "\u{21AD}"), // LEFT RIGHT WAVE ARROW
    ("hcirc", "\u{0125}"), // LATIN SMALL LETTER H WITH CIRCUMFLEX
    ("hearts", "\u{2665}"), // BLACK HEART SUIT
    ("hellip", "\u{2026}"), // HORIZONTAL ELLIPSIS
    ("hercon", "\u{22B9}"), // HERMITIAN CONJUGATE MATRIX
    ("hfr", "\u{1D525}"),  // MATHEMATICAL FRAKTUR SMALL H
    ("hoarr", "\u{21FF}"), // LEFT RIGHT OPEN-HEADED ARROW
    ("homtht", "\u{223B}"), // HOMOTHETIC
    ("hopf", "\u{1D559}"), // MATHEMATICAL DOUBLE-STRUCK SMALL H
    ("horbar", "\u{2015}"), // HORIZONTAL BAR
    ("hscr", "\u{1D4BD}"), // MATHEMATICAL SCRIPT SMALL H
    ("hstrok", "\u{0127}"), // LATIN SMALL LETTER H WITH STROKE
    ("hybull", "\u{2043}"), // HYPHEN BULLET
    ("hyphen", "\u{2010}"), // HYPHEN
    ("iacute", "\u{00ED}"), // LATIN SMALL LETTER I WITH ACUTE
    ("icirc", "\u{00EE}"), // LATIN SMALL LETTER I WITH CIRCUMFLEX
    ("icy", "\u{0438}"),   // CYRILLIC SMALL LETTER I
    ("iecy", "\u{0435}"),  // CYRILLIC SMALL LETTER IE
    ("iexcl", "\u{00A1}"), // INVERTED EXCLAMATION MARK
    ("ifr", "\u{1D526}"),  // MATHEMATICAL FRAKTUR SMALL I
    ("igrave", "\u{00EC}"), // LATIN SMALL LETTER I WITH GRAVE
    ("iinfin", "\u{29DC}"), // INCOMPLETE INFINITY
    ("iiota", "\u{2129}"), // TURNED GREEK SMALL LETTER IOTA
    ("ijlig", "\u{0133}"), // LATIN SMALL LIGATURE IJ
    ("imacr", "\u{012B}"), // LATIN SMALL LETTER I WITH MACRON
    ("image", "\u{2111}"), // BLACK-LETTER CAPITAL I
    ("imath", "\u{0131}"), // LATIN SMALL LETTER DOTLESS I
    ("imof", "\u{22B7}"),  // IMAGE OF
    ("imped", "\u{01B5}"), // LATIN CAPITAL LETTER Z WITH STROKE
    ("incare", "\u{2105}"), // CARE OF
    ("infin", "\u{221E}"), // INFINITY
    ("infintie", "\u{29DD}"), // TIE OVER INFINITY
    ("int", "\u{222B}"),   // INTEGRAL
    ("intcal", "\u{22BA}"), // INTERCALATE
    ("integers", "\u{2124}"), // DOUBLE-STRUCK CAPITAL Z
    ("intlarhk", "\u{2A17}"), // INTEGRAL WITH LEFTWARDS ARROW WITH HOOK
    ("iocy", "\u{0451}"),  // CYRILLIC SMALL LETTER IO
    ("iogon", "\u{012F}"), // LATIN SMALL LETTER I WITH OGONEK
    ("iopf", "\u{1D55A}"), // MATHEMATICAL DOUBLE-STRUCK SMALL I
    ("iota", "\u{03B9}"),  // GREEK SMALL LETTER IOTA
    ("iprod", "\u{2A3C}"), // INTERIOR PRODUCT
    ("iquest", "\u{00BF}"), // INVERTED QUESTION MARK
    ("iscr", "\u{1D4BE}"), // MATHEMATICAL SCRIPT SMALL I
    ("isin", "\u{2208}"),  // ELEMENT OF
    ("isinE", "\u{22F9}"), // ELEMENT OF WITH TWO HORIZONTAL STROKES
    ("isindot", "\u{22F5}"), // ELEMENT OF WITH DOT ABOVE
    ("isins", "\u{22F4}"), // SMALL ELEMENT OF WITH VERTICAL BAR AT END OF HORIZONTAL STROKE
    ("isinsv", "\u{22F3}"), // ELEMENT OF WITH VERTICAL BAR AT END OF HORIZONTAL STROKE
    ("itilde", "\u{0129}"), // LATIN SMALL LETTER I WITH TILDE
    ("iukcy", "\u{0456}"), // CYRILLIC SMALL LETTER BYELORUSSIAN-UKRAINIAN I
    ("iuml", "\u{00EF}"),  // LATIN SMALL LETTER I WITH DIAERESIS
    ("jcirc", "\u{0135}"), // LATIN SMALL LETTER J WITH CIRCUMFLEX
    ("jcy", "\u{0439}"),   // CYRILLIC SMALL LETTER SHORT I
    ("jfr", "\u{1D527}"),  // MATHEMATICAL FRAKTUR SMALL J
    ("jmath", "\u{0237}"), // LATIN SMALL LETTER DOTLESS J
    ("jopf", "\u{1D55B}"), // MATHEMATICAL DOUBLE-STRUCK SMALL J
    ("jscr", "\u{1D4BF}"), // MATHEMATICAL SCRIPT SMALL J
    ("jsercy", "\u{0458}"), // CYRILLIC SMALL LETTER JE
    ("jukcy", "\u{0454}"), // CYRILLIC SMALL LETTER UKRAINIAN IE
    ("kappa", "\u{03BA}"), // GREEK SMALL LETTER KAPPA
    ("kappav", "\u{03F0}"), // GREEK KAPPA SYMBOL
    ("kcedil", "\u{0137}"), // LATIN SMALL LETTER K WITH CEDILLA
    ("kcy", "\u{043A}"),   // CYRILLIC SMALL LETTER KA
    ("kfr", "\u{1D528}"),  // MATHEMATICAL FRAKTUR SMALL K
    ("kgreen", "\u{0138}"), // LATIN SMALL LETTER KRA
    ("khcy", "\u{0445}"),  // CYRILLIC SMALL LETTER HA
    ("kjcy", "\u{045C}"),  // CYRILLIC SMALL LETTER KJE
    ("kopf", "\u{1D55C}"), // MATHEMATICAL DOUBLE-STRUCK SMALL K
    ("kscr", "\u{1D4C0}"), // MATHEMATICAL SCRIPT SMALL K
    ("lAarr", "\u{21DA}"), // LEFTWARDS TRIPLE ARROW
    ("lArr", "\u{21D0}"),  // LEFTWARDS DOUBLE ARROW
    ("lAtail", "\u{291B}"), // LEFTWARDS DOUBLE ARROW-TAIL
    ("lBarr", "\u{290E}"), // LEFTWARDS TRIPLE DASH ARROW
    ("lE", "\u{2266}"),    // LESS-THAN OVER EQUAL TO
    ("lEg", "\u{2A8B}"),   // LESS-THAN ABOVE DOUBLE-LINE EQUAL ABOVE GREATER-THAN
    ("lHar", "\u{2962}"),  // LEFTWARDS HARPOON WITH BARB UP ABOVE LEFTWARDS HARPOON WITH BARB DOWN
    ("lacute", "\u{013A}"), // LATIN SMALL LETTER L WITH ACUTE
    ("laemptyv", "\u{29B4}"), // EMPTY SET WITH LEFT ARROW ABOVE
    ("lambda", "\u{03BB}"), // GREEK SMALL LETTER LAMDA
    ("lang", "\u{27E8}"),  // MATHEMATICAL LEFT ANGLE BRACKET
    ("langd", "\u{2991}"), // LEFT ANGLE BRACKET WITH DOT
    ("lap", "\u{2A85}"),   // LESS-THAN OR APPROXIMATE
    ("laquo", "\u{00AB}"), // LEFT-POINTING DOUBLE ANGLE QUOTATION MARK
    ("larr", "\u{2190}"),  // LEFTWARDS ARROW
    ("larr", "\u{21E4}"),  // LEFTWARDS ARROW TO BAR
    ("larrbfs", "\u{291F}"), // LEFTWARDS ARROW FROM BAR TO BLACK DIAMOND
    ("larrfs", "\u{291D}"), // LEFTWARDS ARROW TO BLACK DIAMOND
    ("larrhk", "\u{21A9}"), // LEFTWARDS ARROW WITH HOOK
    ("larrlp", "\u{21AB}"), // LEFTWARDS ARROW WITH LOOP
    ("larrpl", "\u{2939}"), // LEFT-SIDE ARC ANTICLOCKWISE ARROW
    ("larrsim", "\u{2973}"), // LEFTWARDS ARROW ABOVE TILDE OPERATOR
    ("larrtl", "\u{21A2}"), // LEFTWARDS ARROW WITH TAIL
    ("lat", "\u{2AAB}"),   // LARGER THAN
    ("latail", "\u{2919}"), // LEFTWARDS ARROW-TAIL
    ("late", "\u{2AAD}"),  // LARGER THAN OR EQUAL TO
    ("lbarr", "\u{290C}"), // LEFTWARDS DOUBLE DASH ARROW
    ("lbbrk", "\u{2772}"), // LIGHT LEFT TORTOISE SHELL BRACKET ORNAMENT
    ("lbrke", "\u{298B}"), // LEFT SQUARE BRACKET WITH UNDERBAR
    ("lbrksld", "\u{298F}"), // LEFT SQUARE BRACKET WITH TICK IN BOTTOM CORNER
    ("lbrkslu", "\u{298D}"), // LEFT SQUARE BRACKET WITH TICK IN TOP CORNER
    ("lcaron", "\u{013E}"), // LATIN SMALL LETTER L WITH CARON
    ("lcedil", "\u{013C}"), // LATIN SMALL LETTER L WITH CEDILLA
    ("lceil", "\u{2308}"), // LEFT CEILING
    ("lcu", "\u{007B}"),   // LEFT CURLY BRACKET
    ("lcy", "\u{043B}"),   // CYRILLIC SMALL LETTER EL
    ("ldca", "\u{2936}"),  // ARROW POINTING DOWNWARDS THEN CURVING LEFTWARDS
    ("ldquo", "\u{201C}"), // LEFT DOUBLE QUOTATION MARK
    ("ldquor", "\u{201E}"), // DOUBLE LOW-9 QUOTATION MARK
    ("ldrdhar", "\u{2967}"), /* LEFTWARDS HARPOON WITH BARB DOWN ABOVE RIGHTWARDS HARPOON WITH BARB DOWN */
    ("ldrushar", "\u{294B}"), // LEFT BARB DOWN RIGHT BARB UP HARPOON
    ("ldsh", "\u{21B2}"),    // DOWNWARDS ARROW WITH TIP LEFTWARDS
    ("le", "\u{2264}"),      // LESS-THAN OR EQUAL TO
    ("leg", "\u{22DA}"),     // LESS-THAN EQUAL TO OR GREATER-THAN
    ("les", "\u{2A7D}"),     // LESS-THAN OR SLANTED EQUAL TO
    ("lescc", "\u{2AA8}"),   // LESS-THAN CLOSED BY CURVE ABOVE SLANTED EQUAL
    ("lesdot", "\u{2A7F}"),  // LESS-THAN OR SLANTED EQUAL TO WITH DOT INSIDE
    ("lesdoto", "\u{2A81}"), // LESS-THAN OR SLANTED EQUAL TO WITH DOT ABOVE
    ("lesdotor", "\u{2A83}"), // LESS-THAN OR SLANTED EQUAL TO WITH DOT ABOVE RIGHT
    ("lesges", "\u{2A93}"), /* LESS-THAN ABOVE SLANTED EQUAL ABOVE GREATER-THAN ABOVE SLANTED EQUAL */
    ("lfisht", "\u{297C}"), // LEFT FISH TAIL
    ("lfloor", "\u{230A}"), // LEFT FLOOR
    ("lfr", "\u{1D529}"),   // MATHEMATICAL FRAKTUR SMALL L
    ("lg", "\u{2276}"),     // LESS-THAN OR GREATER-THAN
    ("lgE", "\u{2A91}"),    // LESS-THAN ABOVE GREATER-THAN ABOVE DOUBLE-LINE EQUAL
    ("lhard", "\u{21BD}"),  // LEFTWARDS HARPOON WITH BARB DOWNWARDS
    ("lharu", "\u{21BC}"),  // LEFTWARDS HARPOON WITH BARB UPWARDS
    ("lharul", "\u{296A}"), // LEFTWARDS HARPOON WITH BARB UP ABOVE LONG DASH
    ("lhblk", "\u{2584}"),  // LOWER HALF BLOCK
    ("ljcy", "\u{0459}"),   // CYRILLIC SMALL LETTER LJE
    ("llarr", "\u{21C7}"),  // LEFTWARDS PAIRED ARROWS
    ("llhard", "\u{296B}"), // LEFTWARDS HARPOON WITH BARB DOWN BELOW LONG DASH
    ("lltri", "\u{25FA}"),  // LOWER LEFT TRIANGLE
    ("lmidot", "\u{0140}"), // LATIN SMALL LETTER L WITH MIDDLE DOT
    ("lmoust", "\u{23B0}"), // UPPER LEFT OR LOWER RIGHT CURLY BRACKET SECTION
    ("lnE", "\u{2268}"),    // LESS-THAN BUT NOT EQUAL TO
    ("lnap", "\u{2A89}"),   // LESS-THAN AND NOT APPROXIMATE
    ("lne", "\u{2A87}"),    // LESS-THAN AND SINGLE-LINE NOT EQUAL TO
    ("lnsim", "\u{22E6}"),  // LESS-THAN BUT NOT EQUIVALENT TO
    ("loang", "\u{27EC}"),  // MATHEMATICAL LEFT WHITE TORTOISE SHELL BRACKET
    ("loarr", "\u{21FD}"),  // LEFTWARDS OPEN-HEADED ARROW
    ("lobrk", "\u{27E6}"),  // MATHEMATICAL LEFT WHITE SQUARE BRACKET
    ("lopar", "\u{2985}"),  // LEFT WHITE PARENTHESIS
    ("lopf", "\u{1D55D}"),  // MATHEMATICAL DOUBLE-STRUCK SMALL L
    ("loplus", "\u{2A2D}"), // PLUS SIGN IN LEFT HALF CIRCLE
    ("lotimes", "\u{2A34}"), // MULTIPLICATION SIGN IN LEFT HALF CIRCLE
    ("lowast", "\u{2217}"), // ASTERISK OPERATOR
    ("lowbar", "\u{005F}"), // LOW LINE
    ("loz", "\u{25CA}"),    // LOZENGE
    ("lozf", "\u{29EB}"),   // BLACK LOZENGE
    ("lpar", "\u{0028}"),   // LEFT PARENTHESIS
    ("lparlt", "\u{2993}"), // LEFT ARC LESS-THAN BRACKET
    ("lrarr", "\u{21C6}"),  // LEFTWARDS ARROW OVER RIGHTWARDS ARROW
    ("lrhar", "\u{21CB}"),  // LEFTWARDS HARPOON OVER RIGHTWARDS HARPOON
    ("lrhard", "\u{296D}"), // RIGHTWARDS HARPOON WITH BARB DOWN BELOW LONG DASH
    ("lrm", "\u{200E}"),    // LEFT-TO-RIGHT MARK
    ("lrtri", "\u{22BF}"),  // RIGHT TRIANGLE
    ("lsaquo", "\u{2039}"), // SINGLE LEFT-POINTING ANGLE QUOTATION MARK
    ("lscr", "\u{1D4C1}"),  // MATHEMATICAL SCRIPT SMALL L
    ("lsh", "\u{21B0}"),    // UPWARDS ARROW WITH TIP LEFTWARDS
    ("lsim", "\u{2272}"),   // LESS-THAN OR EQUIVALENT TO
    ("lsime", "\u{2A8D}"),  // LESS-THAN ABOVE SIMILAR OR EQUAL
    ("lsimg", "\u{2A8F}"),  // LESS-THAN ABOVE SIMILAR ABOVE GREATER-THAN
    ("lsq", "\u{005B}"),    // LEFT SQUARE BRACKET
    ("lsquo", "\u{2018}"),  // LEFT SINGLE QUOTATION MARK
    ("lsquor", "\u{201A}"), // SINGLE LOW-9 QUOTATION MARK
    ("lstrok", "\u{0142}"), // LATIN SMALL LETTER L WITH STROKE
    ("lt", "\u{003C}"),     // LESS-THAN SIGN
    ("ltcc", "\u{2AA6}"),   // LESS-THAN CLOSED BY CURVE
    ("ltcir", "\u{2A79}"),  // LESS-THAN WITH CIRCLE INSIDE
    ("ltdot", "\u{22D6}"),  // LESS-THAN WITH DOT
    ("lthree", "\u{22CB}"), // LEFT SEMIDIRECT PRODUCT
    ("ltimes", "\u{22C9}"), // LEFT NORMAL FACTOR SEMIDIRECT PRODUCT
    ("ltlarr", "\u{2976}"), // LESS-THAN ABOVE LEFTWARDS ARROW
    ("ltquest", "\u{2A7B}"), // LESS-THAN WITH QUESTION MARK ABOVE
    ("ltrPar", "\u{2996}"), // DOUBLE RIGHT ARC LESS-THAN BRACKET
    ("ltri", "\u{25C3}"),   // WHITE LEFT-POINTING SMALL TRIANGLE
    ("ltrie", "\u{22B4}"),  // NORMAL SUBGROUP OF OR EQUAL TO
    ("ltrif", "\u{25C2}"),  // BLACK LEFT-POINTING SMALL TRIANGLE
    ("lurdshar", "\u{294A}"), // LEFT BARB UP RIGHT BARB DOWN HARPOON
    ("luruhar", "\u{2966}"), /* LEFTWARDS HARPOON WITH BARB UP ABOVE RIGHTWARDS HARPOON WITH BARB UP */
    ("mDDot", "\u{223A}"),   // GEOMETRIC PROPORTION
    ("macr", "\u{00AF}"),    // MACRON
    ("male", "\u{2642}"),    // MALE SIGN
    ("malt", "\u{2720}"),    // MALTESE CROSS
    ("map", "\u{21A6}"),     // RIGHTWARDS ARROW FROM BAR
    ("marker", "\u{25AE}"),  // BLACK VERTICAL RECTANGLE
    ("mcomma", "\u{2A29}"),  // MINUS SIGN WITH COMMA ABOVE
    ("mcy", "\u{043C}"),     // CYRILLIC SMALL LETTER EM
    ("mdash", "\u{2014}"),   // EM DASH
    ("mfr", "\u{1D52A}"),    // MATHEMATICAL FRAKTUR SMALL M
    ("mho", "\u{2127}"),     // INVERTED OHM SIGN
    ("micro", "\u{00B5}"),   // MICRO SIGN
    ("mid", "\u{2223}"),     // DIVIDES
    ("midcir", "\u{2AF0}"),  // VERTICAL LINE WITH CIRCLE BELOW
    ("middot", "\u{00B7}"),  // MIDDLE DOT
    ("minus", "\u{2212}"),   // MINUS SIGN
    ("minus", "\u{229F}"),   // SQUARED MINUS
    ("minusd", "\u{2238}"),  // DOT MINUS
    ("minusdu", "\u{2A2A}"), // MINUS SIGN WITH DOT BELOW
    ("mlcp", "\u{2ADB}"),    // TRANSVERSAL INTERSECTION
    ("mnplus", "\u{2213}"),  // MINUS-OR-PLUS SIGN
    ("models", "\u{22A7}"),  // MODELS
    ("mopf", "\u{1D55E}"),   // MATHEMATICAL DOUBLE-STRUCK SMALL M
    ("mscr", "\u{1D4C2}"),   // MATHEMATICAL SCRIPT SMALL M
    ("mu", "\u{03BC}"),      // GREEK SMALL LETTER MU
    ("mumap", "\u{22B8}"),   // MULTIMAP
    ("nVDash", "\u{22AF}"),  // NEGATED DOUBLE VERTICAL BAR DOUBLE RIGHT TURNSTILE
    ("nVdash", "\u{22AE}"),  // DOES NOT FORCE
    ("nabla", "\u{2207}"),   // NABLA
    ("nacute", "\u{0144}"),  // LATIN SMALL LETTER N WITH ACUTE
    ("nap", "\u{2249}"),     // NOT ALMOST EQUAL TO
    ("napos", "\u{0149}"),   // LATIN SMALL LETTER N PRECEDED BY APOSTROPHE
    ("natur", "\u{266E}"),   // MUSIC NATURAL SIGN
    ("nbsp", " "),           // NO-BREAK SPACE
    ("ncap", "\u{2A43}"),    // INTERSECTION WITH OVERBAR
    ("ncaron", "\u{0148}"),  // LATIN SMALL LETTER N WITH CARON
    ("ncedil", "\u{0146}"),  // LATIN SMALL LETTER N WITH CEDILLA
    ("ncong", "\u{2247}"),   // NEITHER APPROXIMATELY NOR ACTUALLY EQUAL TO
    ("ncup", "\u{2A42}"),    // UNION WITH OVERBAR
    ("ncy", "\u{043D}"),     // CYRILLIC SMALL LETTER EN
    ("ndash", "\u{2013}"),   // EN DASH
    ("ne", "\u{2260}"),      // NOT EQUAL TO
    ("neArr", "\u{21D7}"),   // NORTH EAST DOUBLE ARROW
    ("nearhk", "\u{2924}"),  // NORTH EAST ARROW WITH HOOK
    ("nearr", "\u{2197}"),   // NORTH EAST ARROW
    ("nequiv", "\u{2262}"),  // NOT IDENTICAL TO
    ("nesear", "\u{2928}"),  // NORTH EAST ARROW AND SOUTH EAST ARROW
    ("nexist", "\u{2204}"),  // THERE DOES NOT EXIST
    ("nfr", "\u{1D52B}"),    // MATHEMATICAL FRAKTUR SMALL N
    ("nge", "\u{2271}"),     // NEITHER GREATER-THAN NOR EQUAL TO
    ("ngsim", "\u{2275}"),   // NEITHER GREATER-THAN NOR EQUIVALENT TO
    ("ngt", "\u{226F}"),     // NOT GREATER-THAN
    ("nhArr", "\u{21CE}"),   // LEFT RIGHT DOUBLE ARROW WITH STROKE
    ("nharr", "\u{21AE}"),   // LEFT RIGHT ARROW WITH STROKE
    ("nhpar", "\u{2AF2}"),   // PARALLEL WITH HORIZONTAL STROKE
    ("nis", "\u{22FC}"),     // SMALL CONTAINS WITH VERTICAL BAR AT END OF HORIZONTAL STROKE
    ("nisd", "\u{22FA}"),    // CONTAINS WITH LONG HORIZONTAL STROKE
    ("niv", "\u{220B}"),     // CONTAINS AS MEMBER
    ("njcy", "\u{045A}"),    // CYRILLIC SMALL LETTER NJE
    ("nlArr", "\u{21CD}"),   // LEFTWARDS DOUBLE ARROW WITH STROKE
    ("nlarr", "\u{219A}"),   // LEFTWARDS ARROW WITH STROKE
    ("nldr", "\u{2025}"),    // TWO DOT LEADER
    ("nle", "\u{2270}"),     // NEITHER LESS-THAN NOR EQUAL TO
    ("nlsim", "\u{2274}"),   // NEITHER LESS-THAN NOR EQUIVALENT TO
    ("nlt", "\u{226E}"),     // NOT LESS-THAN
    ("nltri", "\u{22EA}"),   // NOT NORMAL SUBGROUP OF
    ("nltrie", "\u{22EC}"),  // NOT NORMAL SUBGROUP OF OR EQUAL TO
    ("nmid", "\u{2224}"),    // DOES NOT DIVIDE
    ("nopf", "\u{1D55F}"),   // MATHEMATICAL DOUBLE-STRUCK SMALL N
    ("not", "\u{00AC}"),     // NOT SIGN
    ("notin", "\u{2209}"),   // NOT AN ELEMENT OF
    ("notinv", "\u{22F7}"),  // SMALL ELEMENT OF WITH OVERBAR
    ("notinvc", "\u{22F6}"), // ELEMENT OF WITH OVERBAR
    ("notni", "\u{220C}"),   // DOES NOT CONTAIN AS MEMBER
    ("notniv", "\u{22FE}"),  // SMALL CONTAINS WITH OVERBAR
    ("notnivc", "\u{22FD}"), // CONTAINS WITH OVERBAR
    ("npar", "\u{2226}"),    // NOT PARALLEL TO
    ("npolint", "\u{2A14}"), // LINE INTEGRATION NOT INCLUDING THE POLE
    ("npr", "\u{2280}"),     // DOES NOT PRECEDE
    ("nprcue", "\u{22E0}"),  // DOES NOT PRECEDE OR EQUAL
    ("nrArr", "\u{21CF}"),   // RIGHTWARDS DOUBLE ARROW WITH STROKE
    ("nrarr", "\u{219B}"),   // RIGHTWARDS ARROW WITH STROKE
    ("nrtri", "\u{22EB}"),   // DOES NOT CONTAIN AS NORMAL SUBGROUP
    ("nrtrie", "\u{22ED}"),  // DOES NOT CONTAIN AS NORMAL SUBGROUP OR EQUAL
    ("nsc", "\u{2281}"),     // DOES NOT SUCCEED
    ("nsccue", "\u{22E1}"),  // DOES NOT SUCCEED OR EQUAL
    ("nscr", "\u{1D4C3}"),   // MATHEMATICAL SCRIPT SMALL N
    ("nsim", "\u{2241}"),    // NOT TILDE
    ("nsime", "\u{2244}"),   // NOT ASYMPTOTICALLY EQUAL TO
    ("nsqsube", "\u{22E2}"), // NOT SQUARE IMAGE OF OR EQUAL TO
    ("nsqsupe", "\u{22E3}"), // NOT SQUARE ORIGINAL OF OR EQUAL TO
    ("nsu", "\u{2284}"),     // NOT A SUBSET OF
    ("nsube", "\u{2288}"),   // NEITHER A SUBSET OF NOR EQUAL TO
    ("nsup", "\u{2285}"),    // NOT A SUPERSET OF
    ("nsupe", "\u{2289}"),   // NEITHER A SUPERSET OF NOR EQUAL TO
    ("ntgl", "\u{2279}"),    // NEITHER GREATER-THAN NOR LESS-THAN
    ("ntilde", "\u{00F1}"),  // LATIN SMALL LETTER N WITH TILDE
    ("ntlg", "\u{2278}"),    // NEITHER LESS-THAN NOR GREATER-THAN
    ("nu", "\u{03BD}"),      // GREEK SMALL LETTER NU
    ("num", "\u{0023}"),     // NUMBER SIGN
    ("numero", "\u{2116}"),  // NUMERO SIGN
    ("numsp", "\u{2007}"),   // FIGURE SPACE
    ("nvDash", "\u{22AD}"),  // NOT TRUE
    ("nvHarr", "\u{2904}"),  // LEFT RIGHT DOUBLE ARROW WITH VERTICAL STROKE
    ("nvdash", "\u{22AC}"),  // DOES NOT PROVE
    ("nvinfin", "\u{29DE}"), // INFINITY NEGATED WITH VERTICAL BAR
    ("nvlArr", "\u{2902}"),  // LEFTWARDS DOUBLE ARROW WITH VERTICAL STROKE
    ("nvrArr", "\u{2903}"),  // RIGHTWARDS DOUBLE ARROW WITH VERTICAL STROKE
    ("nwArr", "\u{21D6}"),   // NORTH WEST DOUBLE ARROW
    ("nwarhk", "\u{2923}"),  // NORTH WEST ARROW WITH HOOK
    ("nwarr", "\u{2196}"),   // NORTH WEST ARROW
    ("nwnear", "\u{2927}"),  // NORTH WEST ARROW AND NORTH EAST ARROW
    ("oS", "\u{24C8}"),      // CIRCLED LATIN CAPITAL LETTER S
    ("oacute", "\u{00F3}"),  // LATIN SMALL LETTER O WITH ACUTE
    ("oast", "\u{229B}"),    // CIRCLED ASTERISK OPERATOR
    ("ocir", "\u{229A}"),    // CIRCLED RING OPERATOR
    ("ocirc", "\u{00F4}"),   // LATIN SMALL LETTER O WITH CIRCUMFLEX
    ("ocy", "\u{043E}"),     // CYRILLIC SMALL LETTER O
    ("odash", "\u{229D}"),   // CIRCLED DASH
    ("odblac", "\u{0151}"),  // LATIN SMALL LETTER O WITH DOUBLE ACUTE
    ("odiv", "\u{2A38}"),    // CIRCLED DIVISION SIGN
    ("odot", "\u{2299}"),    // CIRCLED DOT OPERATOR
    ("odsold", "\u{29BC}"),  // CIRCLED ANTICLOCKWISE-ROTATED DIVISION SIGN
    ("oelig", "\u{0153}"),   // LATIN SMALL LIGATURE OE
    ("ofcir", "\u{29BF}"),   // CIRCLED BULLET
    ("ofr", "\u{1D52C}"),    // MATHEMATICAL FRAKTUR SMALL O
    ("ogon", "\u{02DB}"),    // OGONEK
    ("ograve", "\u{00F2}"),  // LATIN SMALL LETTER O WITH GRAVE
    ("ogt", "\u{29C1}"),     // CIRCLED GREATER-THAN
    ("ohbar", "\u{29B5}"),   // CIRCLE WITH HORIZONTAL BAR
    ("ohm", "\u{2126}"),     // OHM SIGN
    ("olarr", "\u{21BA}"),   // ANTICLOCKWISE OPEN CIRCLE ARROW
    ("olcir", "\u{29BE}"),   // CIRCLED WHITE BULLET
    ("olcross", "\u{29BB}"), // CIRCLE WITH SUPERIMPOSED X
    ("oline", "\u{203E}"),   // OVERLINE
    ("olt", "\u{29C0}"),     // CIRCLED LESS-THAN
    ("omacr", "\u{014D}"),   // LATIN SMALL LETTER O WITH MACRON
    ("omega", "\u{03C9}"),   // GREEK SMALL LETTER OMEGA
    ("omicron", "\u{03BF}"), // GREEK SMALL LETTER OMICRON
    ("omid", "\u{29B6}"),    // CIRCLED VERTICAL BAR
    ("ominus", "\u{2296}"),  // CIRCLED MINUS
    ("oopf", "\u{1D560}"),   // MATHEMATICAL DOUBLE-STRUCK SMALL O
    ("opar", "\u{29B7}"),    // CIRCLED PARALLEL
    ("operp", "\u{29B9}"),   // CIRCLED PERPENDICULAR
    ("oplus", "\u{2295}"),   // CIRCLED PLUS
    ("or", "\u{2228}"),      // LOGICAL OR
    ("orarr", "\u{21BB}"),   // CLOCKWISE OPEN CIRCLE ARROW
    ("ord", "\u{2A5D}"),     // LOGICAL OR WITH HORIZONTAL DASH
    ("order", "\u{2134}"),   // SCRIPT SMALL O
    ("ordf", "\u{00AA}"),    // FEMININE ORDINAL INDICATOR
    ("ordm", "\u{00BA}"),    // MASCULINE ORDINAL INDICATOR
    ("origof", "\u{22B6}"),  // ORIGINAL OF
    ("oror", "\u{2A56}"),    // TWO INTERSECTING LOGICAL OR
    ("orslope", "\u{2A57}"), // SLOPING LARGE OR
    ("orv", "\u{2A5B}"),     // LOGICAL OR WITH MIDDLE STEM
    ("oslash", "\u{00F8}"),  // LATIN SMALL LETTER O WITH STROKE
    ("osol", "\u{2298}"),    // CIRCLED DIVISION SLASH
    ("otilde", "\u{00F5}"),  // LATIN SMALL LETTER O WITH TILDE
    ("otimes", "\u{2297}"),  // CIRCLED TIMES
    ("otimesas", "\u{2A36}"), // CIRCLED MULTIPLICATION SIGN WITH CIRCUMFLEX ACCENT
    ("ouml", "\u{00F6}"),    // LATIN SMALL LETTER O WITH DIAERESIS
    ("ovbar", "\u{233D}"),   // APL FUNCTIONAL SYMBOL CIRCLE STILE
    ("par", "\u{2225}"),     // PARALLEL TO
    ("para", "\u{00B6}"),    // PILCROW SIGN
    ("parsim", "\u{2AF3}"),  // PARALLEL WITH TILDE OPERATOR
    ("parsl", "\u{2AFD}"),   // DOUBLE SOLIDUS OPERATOR
    ("part", "\u{2202}"),    // PARTIAL DIFFERENTIAL
    ("pcy", "\u{043F}"),     // CYRILLIC SMALL LETTER PE
    ("percnt", "\u{0025}"),  // PERCENT SIGN
    ("period", "\u{002E}"),  // FULL STOP
    ("permil", "\u{2030}"),  // PER MILLE SIGN
    ("pertenk", "\u{2031}"), // PER TEN THOUSAND SIGN
    ("pfr", "\u{1D52D}"),    // MATHEMATICAL FRAKTUR SMALL P
    ("phi", "\u{03C6}"),     // GREEK SMALL LETTER PHI
    ("phmmat", "\u{2133}"),  // SCRIPT CAPITAL M
    ("phone", "\u{260E}"),   // BLACK TELEPHONE
    ("pi", "\u{03C0}"),      // GREEK SMALL LETTER PI
    ("piv", "\u{03D6}"),     // GREEK PI SYMBOL
    ("planck", "\u{210F}"),  // PLANCK CONSTANT OVER TWO PI
    ("planckh", "\u{210E}"), // PLANCK CONSTANT
    ("plus", "\u{002B}"),    // PLUS SIGN
    ("plusacir", "\u{2A23}"), // PLUS SIGN WITH CIRCUMFLEX ACCENT ABOVE
    ("plus", "\u{229E}"),    // SQUARED PLUS
    ("pluscir", "\u{2A22}"), // PLUS SIGN WITH SMALL CIRCLE ABOVE
    ("plusdo", "\u{2214}"),  // DOT PLUS
    ("plusdu", "\u{2A25}"),  // PLUS SIGN WITH DOT BELOW
    ("pluse", "\u{2A72}"),   // PLUS SIGN ABOVE EQUALS SIGN
    ("plusmn", "\u{00B1}"),  // PLUS-MINUS SIGN
    ("plussim", "\u{2A26}"), // PLUS SIGN WITH TILDE BELOW
    ("plustwo", "\u{2A27}"), // PLUS SIGN WITH SUBSCRIPT TWO
    ("pointint", "\u{2A15}"), // INTEGRAL AROUND A POINT OPERATOR
    ("popf", "\u{1D561}"),   // MATHEMATICAL DOUBLE-STRUCK SMALL P
    ("pound", "\u{00A3}"),   // POUND SIGN
    ("pr", "\u{227A}"),      // PRECEDES
    ("prE", "\u{2AB3}"),     // PRECEDES ABOVE EQUALS SIGN
    ("prap", "\u{2AB7}"),    // PRECEDES ABOVE ALMOST EQUAL TO
    ("prcue", "\u{227C}"),   // PRECEDES OR EQUAL TO
    ("pre", "\u{2AAF}"),     // PRECEDES ABOVE SINGLE-LINE EQUALS SIGN
    ("prime", "\u{2032}"),   // PRIME
    ("prnE", "\u{2AB5}"),    // PRECEDES ABOVE NOT EQUAL TO
    ("prnap", "\u{2AB9}"),   // PRECEDES ABOVE NOT ALMOST EQUAL TO
    ("prnsim", "\u{22E8}"),  // PRECEDES BUT NOT EQUIVALENT TO
    ("prod", "\u{220F}"),    // N-ARY PRODUCT
    ("profalar", "\u{232E}"), // ALL AROUND-PROFILE
    ("profline", "\u{2312}"), // ARC
    ("profsurf", "\u{2313}"), // SEGMENT
    ("prop", "\u{221D}"),    // PROPORTIONAL TO
    ("prsim", "\u{227E}"),   // PRECEDES OR EQUIVALENT TO
    ("prurel", "\u{22B0}"),  // PRECEDES UNDER RELATION
    ("pscr", "\u{1D4C5}"),   // MATHEMATICAL SCRIPT SMALL P
    ("psi", "\u{03C8}"),     // GREEK SMALL LETTER PSI
    ("puncsp", "\u{2008}"),  // PUNCTUATION SPACE
    ("qfr", "\u{1D52E}"),    // MATHEMATICAL FRAKTUR SMALL Q
    ("qint", "\u{2A0C}"),    // QUADRUPLE INTEGRAL OPERATOR
    ("qopf", "\u{1D562}"),   // MATHEMATICAL DOUBLE-STRUCK SMALL Q
    ("qprime", "\u{2057}"),  // QUADRUPLE PRIME
    ("qscr", "\u{1D4C6}"),   // MATHEMATICAL SCRIPT SMALL Q
    ("quaternions", "\u{210D}"), // DOUBLE-STRUCK CAPITAL H
    ("quatint", "\u{2A16}"), // QUATERNION INTEGRAL OPERATOR
    ("quest", "\u{003F}"),   // QUESTION MARK
    ("quot", "\u{0022}"),    // QUOTATION MARK
    ("rAarr", "\u{21DB}"),   // RIGHTWARDS TRIPLE ARROW
    ("rArr", "\u{21D2}"),    // RIGHTWARDS DOUBLE ARROW
    ("rAtail", "\u{291C}"),  // RIGHTWARDS DOUBLE ARROW-TAIL
    ("rBarr", "\u{290F}"),   // RIGHTWARDS TRIPLE DASH ARROW
    ("rHar", "\u{2964}"), /* RIGHTWARDS HARPOON WITH BARB UP ABOVE RIGHTWARDS HARPOON WITH BARB DOWN */
    ("race", "\u{29DA}"), // LEFT DOUBLE WIGGLY FENCE
    ("racute", "\u{0155}"), // LATIN SMALL LETTER R WITH ACUTE
    ("radic", "\u{221A}"), // SQUARE ROOT
    ("raemptyv", "\u{29B3}"), // EMPTY SET WITH RIGHT ARROW ABOVE
    ("rang", "\u{27E9}"), // MATHEMATICAL RIGHT ANGLE BRACKET
    ("rangd", "\u{2992}"), // RIGHT ANGLE BRACKET WITH DOT
    ("range", "\u{29A5}"), // REVERSED ANGLE WITH UNDERBAR
    ("raquo", "\u{00BB}"), // RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK
    ("rarr", "\u{2192}"), // RIGHTWARDS ARROW
    ("rarrap", "\u{2975}"), // RIGHTWARDS ARROW ABOVE ALMOST EQUAL TO
    ("rarr", "\u{21E5}"), // RIGHTWARDS ARROW TO BAR
    ("rarrbfs", "\u{2920}"), // RIGHTWARDS ARROW FROM BAR TO BLACK DIAMOND
    ("rarrc", "\u{2933}"), // WAVE ARROW POINTING DIRECTLY RIGHT
    ("rarrfs", "\u{291E}"), // RIGHTWARDS ARROW TO BLACK DIAMOND
    ("rarrhk", "\u{21AA}"), // RIGHTWARDS ARROW WITH HOOK
    ("rarrlp", "\u{21AC}"), // RIGHTWARDS ARROW WITH LOOP
    ("rarrpl", "\u{2945}"), // RIGHTWARDS ARROW WITH PLUS BELOW
    ("rarrsim", "\u{2974}"), // RIGHTWARDS ARROW ABOVE TILDE OPERATOR
    ("rarrtl", "\u{21A3}"), // RIGHTWARDS ARROW WITH TAIL
    ("rarrw", "\u{219D}"), // RIGHTWARDS WAVE ARROW
    ("ratail", "\u{291A}"), // RIGHTWARDS ARROW-TAIL
    ("ratio", "\u{2236}"), // RATIO
    ("rationals", "\u{211A}"), // DOUBLE-STRUCK CAPITAL Q
    ("rbarr", "\u{290D}"), // RIGHTWARDS DOUBLE DASH ARROW
    ("rbbrk", "\u{2773}"), // LIGHT RIGHT TORTOISE SHELL BRACKET ORNAMENT
    ("rbrke", "\u{298C}"), // RIGHT SQUARE BRACKET WITH UNDERBAR
    ("rbrksld", "\u{298E}"), // RIGHT SQUARE BRACKET WITH TICK IN BOTTOM CORNER
    ("rbrkslu", "\u{2990}"), // RIGHT SQUARE BRACKET WITH TICK IN TOP CORNER
    ("rcaron", "\u{0159}"), // LATIN SMALL LETTER R WITH CARON
    ("rcedil", "\u{0157}"), // LATIN SMALL LETTER R WITH CEDILLA
    ("rceil", "\u{2309}"), // RIGHT CEILING
    ("rcu", "\u{007D}"),  // RIGHT CURLY BRACKET
    ("rcy", "\u{0440}"),  // CYRILLIC SMALL LETTER ER
    ("rdca", "\u{2937}"), // ARROW POINTING DOWNWARDS THEN CURVING RIGHTWARDS
    ("rdldhar", "\u{2969}"), /* RIGHTWARDS HARPOON WITH BARB DOWN ABOVE LEFTWARDS HARPOON WITH BARB DOWN */
    ("rdquo", "\u{201D}"),   // RIGHT DOUBLE QUOTATION MARK
    ("rdsh", "\u{21B3}"),    // DOWNWARDS ARROW WITH TIP RIGHTWARDS
    ("real", "\u{211C}"),    // BLACK-LETTER CAPITAL R
    ("reals", "\u{211D}"),   // DOUBLE-STRUCK CAPITAL R
    ("rect", "\u{25AD}"),    // WHITE RECTANGLE
    ("reg", "\u{00AE}"),     // REGISTERED SIGN
    ("rfisht", "\u{297D}"),  // RIGHT FISH TAIL
    ("rfloor", "\u{230B}"),  // RIGHT FLOOR
    ("rfr", "\u{1D52F}"),    // MATHEMATICAL FRAKTUR SMALL R
    ("rhard", "\u{21C1}"),   // RIGHTWARDS HARPOON WITH BARB DOWNWARDS
    ("rharu", "\u{21C0}"),   // RIGHTWARDS HARPOON WITH BARB UPWARDS
    ("rharul", "\u{296C}"),  // RIGHTWARDS HARPOON WITH BARB UP ABOVE LONG DASH
    ("rho", "\u{03C1}"),     // GREEK SMALL LETTER RHO
    ("rhov", "\u{03F1}"),    // GREEK RHO SYMBOL
    ("ring", "\u{02DA}"),    // RING ABOVE
    ("rlarr", "\u{21C4}"),   // RIGHTWARDS ARROW OVER LEFTWARDS ARROW
    ("rlhar", "\u{21CC}"),   // RIGHTWARDS HARPOON OVER LEFTWARDS HARPOON
    ("rlm", "\u{200F}"),     // RIGHT-TO-LEFT MARK
    ("rmoust", "\u{23B1}"),  // UPPER RIGHT OR LOWER LEFT CURLY BRACKET SECTION
    ("rnmid", "\u{2AEE}"),   // DOES NOT DIVIDE WITH REVERSED NEGATION SLASH
    ("roang", "\u{27ED}"),   // MATHEMATICAL RIGHT WHITE TORTOISE SHELL BRACKET
    ("roarr", "\u{21FE}"),   // RIGHTWARDS OPEN-HEADED ARROW
    ("robrk", "\u{27E7}"),   // MATHEMATICAL RIGHT WHITE SQUARE BRACKET
    ("ropar", "\u{2986}"),   // RIGHT WHITE PARENTHESIS
    ("ropf", "\u{1D563}"),   // MATHEMATICAL DOUBLE-STRUCK SMALL R
    ("roplus", "\u{2A2E}"),  // PLUS SIGN IN RIGHT HALF CIRCLE
    ("rotimes", "\u{2A35}"), // MULTIPLICATION SIGN IN RIGHT HALF CIRCLE
    ("rpar", "\u{0029}"),    // RIGHT PARENTHESIS
    ("rpargt", "\u{2994}"),  // RIGHT ARC GREATER-THAN BRACKET
    ("rppolint", "\u{2A12}"), // LINE INTEGRATION WITH RECTANGULAR PATH AROUND POLE
    ("rrarr", "\u{21C9}"),   // RIGHTWARDS PAIRED ARROWS
    ("rsaquo", "\u{203A}"),  // SINGLE RIGHT-POINTING ANGLE QUOTATION MARK
    ("rscr", "\u{1D4C7}"),   // MATHEMATICAL SCRIPT SMALL R
    ("rsh", "\u{21B1}"),     // UPWARDS ARROW WITH TIP RIGHTWARDS
    ("rsq", "\u{005D}"),     // RIGHT SQUARE BRACKET
    ("rsquo", "\u{2019}"),   // RIGHT SINGLE QUOTATION MARK
    ("rthree", "\u{22CC}"),  // RIGHT SEMIDIRECT PRODUCT
    ("rtimes", "\u{22CA}"),  // RIGHT NORMAL FACTOR SEMIDIRECT PRODUCT
    ("rtri", "\u{25B9}"),    // WHITE RIGHT-POINTING SMALL TRIANGLE
    ("rtrie", "\u{22B5}"),   // CONTAINS AS NORMAL SUBGROUP OR EQUAL TO
    ("rtrif", "\u{25B8}"),   // BLACK RIGHT-POINTING SMALL TRIANGLE
    ("rtriltri", "\u{29CE}"), // RIGHT TRIANGLE ABOVE LEFT TRIANGLE
    ("ruluhar", "\u{2968}"), /* RIGHTWARDS HARPOON WITH BARB UP ABOVE LEFTWARDS HARPOON WITH BARB UP */
    ("rx", "\u{211E}"),      // PRESCRIPTION TAKE
    ("sacute", "\u{015B}"),  // LATIN SMALL LETTER S WITH ACUTE
    ("sc", "\u{227B}"),      // SUCCEEDS
    ("scE", "\u{2AB4}"),     // SUCCEEDS ABOVE EQUALS SIGN
    ("scap", "\u{2AB8}"),    // SUCCEEDS ABOVE ALMOST EQUAL TO
    ("scaron", "\u{0161}"),  // LATIN SMALL LETTER S WITH CARON
    ("sccue", "\u{227D}"),   // SUCCEEDS OR EQUAL TO
    ("sce", "\u{2AB0}"),     // SUCCEEDS ABOVE SINGLE-LINE EQUALS SIGN
    ("scedil", "\u{015F}"),  // LATIN SMALL LETTER S WITH CEDILLA
    ("scirc", "\u{015D}"),   // LATIN SMALL LETTER S WITH CIRCUMFLEX
    ("scnE", "\u{2AB6}"),    // SUCCEEDS ABOVE NOT EQUAL TO
    ("scnap", "\u{2ABA}"),   // SUCCEEDS ABOVE NOT ALMOST EQUAL TO
    ("scnsim", "\u{22E9}"),  // SUCCEEDS BUT NOT EQUIVALENT TO
    ("scpolint", "\u{2A13}"), // LINE INTEGRATION WITH SEMICIRCULAR PATH AROUND POLE
    ("scsim", "\u{227F}"),   // SUCCEEDS OR EQUIVALENT TO
    ("scy", "\u{0441}"),     // CYRILLIC SMALL LETTER ES
    ("sdot", "\u{22C5}"),    // DOT OPERATOR
    ("sdot", "\u{22A1}"),    // SQUARED DOT OPERATOR
    ("sdote", "\u{2A66}"),   // EQUALS SIGN WITH DOT BELOW
    ("seArr", "\u{21D8}"),   // SOUTH EAST DOUBLE ARROW
    ("searhk", "\u{2925}"),  // SOUTH EAST ARROW WITH HOOK
    ("searr", "\u{2198}"),   // SOUTH EAST ARROW
    ("sect", "\u{00A7}"),    // SECTION SIGN
    ("semi", "\u{003B}"),    // SEMICOLON
    ("seswar", "\u{2929}"),  // SOUTH EAST ARROW AND SOUTH WEST ARROW
    ("setmn", "\u{2216}"),   // SET MINUS
    ("sext", "\u{2736}"),    // SIX POINTED BLACK STAR
    ("sfr", "\u{1D530}"),    // MATHEMATICAL FRAKTUR SMALL S
    ("sharp", "\u{266F}"),   // MUSIC SHARP SIGN
    ("shchcy", "\u{0449}"),  // CYRILLIC SMALL LETTER SHCHA
    ("shcy", "\u{0448}"),    // CYRILLIC SMALL LETTER SHA
    ("shy", "\u{00AD}"),     // SOFT HYPHEN
    ("sigma", "\u{03C3}"),   // GREEK SMALL LETTER SIGMA
    ("sigmav", "\u{03C2}"),  // GREEK SMALL LETTER FINAL SIGMA
    ("sim", "\u{223C}"),     // TILDE OPERATOR
    ("simdot", "\u{2A6A}"),  // TILDE OPERATOR WITH DOT ABOVE
    ("sime", "\u{2243}"),    // ASYMPTOTICALLY EQUAL TO
    ("simg", "\u{2A9E}"),    // SIMILAR OR GREATER-THAN
    ("simgE", "\u{2AA0}"),   // SIMILAR ABOVE GREATER-THAN ABOVE EQUALS SIGN
    ("siml", "\u{2A9D}"),    // SIMILAR OR LESS-THAN
    ("simlE", "\u{2A9F}"),   // SIMILAR ABOVE LESS-THAN ABOVE EQUALS SIGN
    ("simne", "\u{2246}"),   // APPROXIMATELY BUT NOT ACTUALLY EQUAL TO
    ("simplus", "\u{2A24}"), // PLUS SIGN WITH TILDE ABOVE
    ("simrarr", "\u{2972}"), // TILDE OPERATOR ABOVE RIGHTWARDS ARROW
    ("smashp", "\u{2A33}"),  // SMASH PRODUCT
    ("smeparsl", "\u{29E4}"), // EQUALS SIGN AND SLANTED PARALLEL WITH TILDE ABOVE
    ("smile", "\u{2323}"),   // SMILE
    ("smt", "\u{2AAA}"),     // SMALLER THAN
    ("smte", "\u{2AAC}"),    // SMALLER THAN OR EQUAL TO
    ("softcy", "\u{044C}"),  // CYRILLIC SMALL LETTER SOFT SIGN
    ("sol", "\u{002F}"),     // SOLIDUS
    ("sol", "\u{29C4}"),     // SQUARED RISING DIAGONAL SLASH
    ("solbar", "\u{233F}"),  // APL FUNCTIONAL SYMBOL SLASH BAR
    ("sopf", "\u{1D564}"),   // MATHEMATICAL DOUBLE-STRUCK SMALL S
    ("spades", "\u{2660}"),  // BLACK SPADE SUIT
    ("sqcap", "\u{2293}"),   // SQUARE CAP
    ("sqcup", "\u{2294}"),   // SQUARE CUP
    ("sqsu", "\u{228F}"),    // SQUARE IMAGE OF
    ("sqsube", "\u{2291}"),  // SQUARE IMAGE OF OR EQUAL TO
    ("sqsup", "\u{2290}"),   // SQUARE ORIGINAL OF
    ("sqsupe", "\u{2292}"),  // SQUARE ORIGINAL OF OR EQUAL TO
    ("squ", "\u{25A1}"),     // WHITE SQUARE
    ("squf", "\u{25AA}"),    // BLACK SMALL SQUARE
    ("sscr", "\u{1D4C8}"),   // MATHEMATICAL SCRIPT SMALL S
    ("sstarf", "\u{22C6}"),  // STAR OPERATOR
    ("star", "\u{2606}"),    // WHITE STAR
    ("starf", "\u{2605}"),   // BLACK STAR
    ("straightphi", "\u{03D5}"), // GREEK PHI SYMBOL
    ("su", "\u{2282}"),      // SUBSET OF
    ("subE", "\u{2AC5}"),    // SUBSET OF ABOVE EQUALS SIGN
    ("subdot", "\u{2ABD}"),  // SUBSET WITH DOT
    ("sube", "\u{2286}"),    // SUBSET OF OR EQUAL TO
    ("subedot", "\u{2AC3}"), // SUBSET OF OR EQUAL TO WITH DOT ABOVE
    ("submult", "\u{2AC1}"), // SUBSET WITH MULTIPLICATION SIGN BELOW
    ("subnE", "\u{2ACB}"),   // SUBSET OF ABOVE NOT EQUAL TO
    ("subne", "\u{228A}"),   // SUBSET OF WITH NOT EQUAL TO
    ("subplus", "\u{2ABF}"), // SUBSET WITH PLUS SIGN BELOW
    ("subrarr", "\u{2979}"), // SUBSET ABOVE RIGHTWARDS ARROW
    ("subsim", "\u{2AC7}"),  // SUBSET OF ABOVE TILDE OPERATOR
    ("subsu", "\u{2AD5}"),   // SUBSET ABOVE SUBSET
    ("subsup", "\u{2AD3}"),  // SUBSET ABOVE SUPERSET
    ("sum", "\u{2211}"),     // N-ARY SUMMATION
    ("sung", "\u{266A}"),    // EIGHTH NOTE
    ("sup", "\u{2283}"),     // SUPERSET OF
    ("sup1", "\u{00B9}"),    // SUPERSCRIPT ONE
    ("sup2", "\u{00B2}"),    // SUPERSCRIPT TWO
    ("sup3", "\u{00B3}"),    // SUPERSCRIPT THREE
    ("supE", "\u{2AC6}"),    // SUPERSET OF ABOVE EQUALS SIGN
    ("supdot", "\u{2ABE}"),  // SUPERSET WITH DOT
    ("supdsu", "\u{2AD8}"),  // SUPERSET BESIDE AND JOINED BY DASH WITH SUBSET
    ("supe", "\u{2287}"),    // SUPERSET OF OR EQUAL TO
    ("supedot", "\u{2AC4}"), // SUPERSET OF OR EQUAL TO WITH DOT ABOVE
    ("suphsu", "\u{2AD7}"),  // SUPERSET BESIDE SUBSET
    ("suplarr", "\u{297B}"), // SUPERSET ABOVE LEFTWARDS ARROW
    ("supmult", "\u{2AC2}"), // SUPERSET WITH MULTIPLICATION SIGN BELOW
    ("supnE", "\u{2ACC}"),   // SUPERSET OF ABOVE NOT EQUAL TO
    ("supne", "\u{228B}"),   // SUPERSET OF WITH NOT EQUAL TO
    ("supplus", "\u{2AC0}"), // SUPERSET WITH PLUS SIGN BELOW
    ("supsim", "\u{2AC8}"),  // SUPERSET OF ABOVE TILDE OPERATOR
    ("supsu", "\u{2AD4}"),   // SUPERSET ABOVE SUBSET
    ("supsup", "\u{2AD6}"),  // SUPERSET ABOVE SUPERSET
    ("swArr", "\u{21D9}"),   // SOUTH WEST DOUBLE ARROW
    ("swarhk", "\u{2926}"),  // SOUTH WEST ARROW WITH HOOK
    ("swarr", "\u{2199}"),   // SOUTH WEST ARROW
    ("swnwar", "\u{292A}"),  // SOUTH WEST ARROW AND NORTH WEST ARROW
    ("szlig", "\u{00DF}"),   // LATIN SMALL LETTER SHARP S
    ("target", "\u{2316}"),  // POSITION INDICATOR
    ("tau", "\u{03C4}"),     // GREEK SMALL LETTER TAU
    ("tbrk", "\u{23B4}"),    // TOP SQUARE BRACKET
    ("tcaron", "\u{0165}"),  // LATIN SMALL LETTER T WITH CARON
    ("tcedil", "\u{0163}"),  // LATIN SMALL LETTER T WITH CEDILLA
    ("tcy", "\u{0442}"),     // CYRILLIC SMALL LETTER TE
    ("tdot", "\u{20DB}"),    // COMBINING THREE DOTS ABOVE
    ("telrec", "\u{2315}"),  // TELEPHONE RECORDER
    ("tfr", "\u{1D531}"),    // MATHEMATICAL FRAKTUR SMALL T
    ("there4", "\u{2234}"),  // THEREFORE
    ("theta", "\u{03B8}"),   // GREEK SMALL LETTER THETA
    ("thetav", "\u{03D1}"),  // GREEK THETA SYMBOL
    ("thinsp", "\u{2009}"),  // THIN SPACE
    ("thorn", "\u{00FE}"),   // LATIN SMALL LETTER THORN
    ("tilde", "\u{02DC}"),   // SMALL TILDE
    ("times", "\u{00D7}"),   // MULTIPLICATION SIGN
    ("times", "\u{22A0}"),   // SQUARED TIMES
    ("timesbar", "\u{2A31}"), // MULTIPLICATION SIGN WITH UNDERBAR
    ("timesd", "\u{2A30}"),  // MULTIPLICATION SIGN WITH DOT ABOVE
    ("tint", "\u{222D}"),    // TRIPLE INTEGRAL
    ("top", "\u{22A4}"),     // DOWN TACK
    ("topbot", "\u{2336}"),  // APL FUNCTIONAL SYMBOL I-BEAM
    ("topcir", "\u{2AF1}"),  // DOWN TACK WITH CIRCLE BELOW
    ("topf", "\u{1D565}"),   // MATHEMATICAL DOUBLE-STRUCK SMALL T
    ("topfork", "\u{2ADA}"), // PITCHFORK WITH TEE TOP
    ("tprime", "\u{2034}"),  // TRIPLE PRIME
    ("trade", "\u{2122}"),   // TRADE MARK SIGN
    ("tridot", "\u{25EC}"),  // WHITE UP-POINTING TRIANGLE WITH DOT
    ("trie", "\u{225C}"),    // DELTA EQUAL TO
    ("triminus", "\u{2A3A}"), // MINUS SIGN IN TRIANGLE
    ("triplus", "\u{2A39}"), // PLUS SIGN IN TRIANGLE
    ("tris", "\u{29CD}"),    // TRIANGLE WITH SERIFS AT BOTTOM
    ("tritime", "\u{2A3B}"), // MULTIPLICATION SIGN IN TRIANGLE
    ("trpezium", "\u{23E2}"), // WHITE TRAPEZIUM
    ("tscr", "\u{1D4C9}"),   // MATHEMATICAL SCRIPT SMALL T
    ("tscy", "\u{0446}"),    // CYRILLIC SMALL LETTER TSE
    ("tshcy", "\u{045B}"),   // CYRILLIC SMALL LETTER TSHE
    ("tstrok", "\u{0167}"),  // LATIN SMALL LETTER T WITH STROKE
    ("twixt", "\u{226C}"),   // BETWEEN
    ("uArr", "\u{21D1}"),    // UPWARDS DOUBLE ARROW
    ("uHar", "\u{2963}"), // UPWARDS HARPOON WITH BARB LEFT BESIDE UPWARDS HARPOON WITH BARB RIGHT
    ("uacute", "\u{00FA}"), // LATIN SMALL LETTER U WITH ACUTE
    ("uarr", "\u{2191}"), // UPWARDS ARROW
    ("ubrcy", "\u{045E}"), // CYRILLIC SMALL LETTER SHORT U
    ("ubreve", "\u{016D}"), // LATIN SMALL LETTER U WITH BREVE
    ("ucirc", "\u{00FB}"), // LATIN SMALL LETTER U WITH CIRCUMFLEX
    ("ucy", "\u{0443}"),  // CYRILLIC SMALL LETTER U
    ("udarr", "\u{21C5}"), // UPWARDS ARROW LEFTWARDS OF DOWNWARDS ARROW
    ("udblac", "\u{0171}"), // LATIN SMALL LETTER U WITH DOUBLE ACUTE
    ("udhar", "\u{296E}"), /* UPWARDS HARPOON WITH BARB LEFT BESIDE DOWNWARDS HARPOON WITH BARB RIGHT */
    ("ufisht", "\u{297E}"), // UP FISH TAIL
    ("ufr", "\u{1D532}"),  // MATHEMATICAL FRAKTUR SMALL U
    ("ugrave", "\u{00F9}"), // LATIN SMALL LETTER U WITH GRAVE
    ("uharl", "\u{21BF}"), // UPWARDS HARPOON WITH BARB LEFTWARDS
    ("uharr", "\u{21BE}"), // UPWARDS HARPOON WITH BARB RIGHTWARDS
    ("uhblk", "\u{2580}"), // UPPER HALF BLOCK
    ("ulcorn", "\u{231C}"), // TOP LEFT CORNER
    ("ulcrop", "\u{230F}"), // TOP LEFT CROP
    ("ultri", "\u{25F8}"), // UPPER LEFT TRIANGLE
    ("umacr", "\u{016B}"), // LATIN SMALL LETTER U WITH MACRON
    ("uogon", "\u{0173}"), // LATIN SMALL LETTER U WITH OGONEK
    ("uopf", "\u{1D566}"), // MATHEMATICAL DOUBLE-STRUCK SMALL U
    ("uplus", "\u{228E}"), // MULTISET UNION
    ("upsi", "\u{03C5}"),  // GREEK SMALL LETTER UPSILON
    ("urcorn", "\u{231D}"), // TOP RIGHT CORNER
    ("urcrop", "\u{230E}"), // TOP RIGHT CROP
    ("uring", "\u{016F}"), // LATIN SMALL LETTER U WITH RING ABOVE
    ("urtri", "\u{25F9}"), // UPPER RIGHT TRIANGLE
    ("uscr", "\u{1D4CA}"), // MATHEMATICAL SCRIPT SMALL U
    ("utdot", "\u{22F0}"), // UP RIGHT DIAGONAL ELLIPSIS
    ("utilde", "\u{0169}"), // LATIN SMALL LETTER U WITH TILDE
    ("utri", "\u{25B5}"),  // WHITE UP-POINTING SMALL TRIANGLE
    ("utrif", "\u{25B4}"), // BLACK UP-POINTING SMALL TRIANGLE
    ("uuarr", "\u{21C8}"), // UPWARDS PAIRED ARROWS
    ("uuml", "\u{00FC}"),  // LATIN SMALL LETTER U WITH DIAERESIS
    ("uwangle", "\u{29A7}"), // OBLIQUE ANGLE OPENING DOWN
    ("vArr", "\u{21D5}"),  // UP DOWN DOUBLE ARROW
    ("vBar", "\u{2AE8}"),  // SHORT UP TACK WITH UNDERBAR
    ("vBarv", "\u{2AE9}"), // SHORT UP TACK ABOVE SHORT DOWN TACK
    ("vDash", "\u{22A8}"), // TRUE
    ("vangrt", "\u{299C}"), // RIGHT ANGLE VARIANT WITH SQUARE
    ("varr", "\u{2195}"),  // UP DOWN ARROW
    ("vcy", "\u{0432}"),   // CYRILLIC SMALL LETTER VE
    ("vdash", "\u{22A2}"), // RIGHT TACK
    ("veebar", "\u{22BB}"), // XOR
    ("veeeq", "\u{225A}"), // EQUIANGULAR TO
    ("vellip", "\u{22EE}"), // VERTICAL ELLIPSIS
    ("verbar", "\u{007C}"), // VERTICAL LINE
    ("vfr", "\u{1D533}"),  // MATHEMATICAL FRAKTUR SMALL V
    ("vltri", "\u{22B2}"), // NORMAL SUBGROUP OF
    ("vopf", "\u{1D567}"), // MATHEMATICAL DOUBLE-STRUCK SMALL V
    ("vrtri", "\u{22B3}"), // CONTAINS AS NORMAL SUBGROUP
    ("vscr", "\u{1D4CB}"), // MATHEMATICAL SCRIPT SMALL V
    ("vzigzag", "\u{299A}"), // VERTICAL ZIGZAG LINE
    ("wcirc", "\u{0175}"), // LATIN SMALL LETTER W WITH CIRCUMFLEX
    ("wedbar", "\u{2A5F}"), // LOGICAL AND WITH UNDERBAR
    ("wedgeq", "\u{2259}"), // ESTIMATES
    ("weierp", "\u{2118}"), // SCRIPT CAPITAL P
    ("wfr", "\u{1D534}"),  // MATHEMATICAL FRAKTUR SMALL W
    ("wopf", "\u{1D568}"), // MATHEMATICAL DOUBLE-STRUCK SMALL W
    ("wreath", "\u{2240}"), // WREATH PRODUCT
    ("wscr", "\u{1D4CC}"), // MATHEMATICAL SCRIPT SMALL W
    ("xcap", "\u{22C2}"),  // N-ARY INTERSECTION
    ("xcirc", "\u{25EF}"), // LARGE CIRCLE
    ("xcup", "\u{22C3}"),  // N-ARY UNION
    ("xdtri", "\u{25BD}"), // WHITE DOWN-POINTING TRIANGLE
    ("xfr", "\u{1D535}"),  // MATHEMATICAL FRAKTUR SMALL X
    ("xhArr", "\u{27FA}"), // LONG LEFT RIGHT DOUBLE ARROW
    ("xharr", "\u{27F7}"), // LONG LEFT RIGHT ARROW
    ("xi", "\u{03BE}"),    // GREEK SMALL LETTER XI
    ("xlArr", "\u{27F8}"), // LONG LEFTWARDS DOUBLE ARROW
    ("xlarr", "\u{27F5}"), // LONG LEFTWARDS ARROW
    ("xmap", "\u{27FC}"),  // LONG RIGHTWARDS ARROW FROM BAR
    ("xnis", "\u{22FB}"),  // CONTAINS WITH VERTICAL BAR AT END OF HORIZONTAL STROKE
    ("xodot", "\u{2A00}"), // N-ARY CIRCLED DOT OPERATOR
    ("xopf", "\u{1D569}"), // MATHEMATICAL DOUBLE-STRUCK SMALL X
    ("xoplus", "\u{2A01}"), // N-ARY CIRCLED PLUS OPERATOR
    ("xotime", "\u{2A02}"), // N-ARY CIRCLED TIMES OPERATOR
    ("xrArr", "\u{27F9}"), // LONG RIGHTWARDS DOUBLE ARROW
    ("xrarr", "\u{27F6}"), // LONG RIGHTWARDS ARROW
    ("xscr", "\u{1D4CD}"), // MATHEMATICAL SCRIPT SMALL X
    ("xsqcup", "\u{2A06}"), // N-ARY SQUARE UNION OPERATOR
    ("xuplus", "\u{2A04}"), // N-ARY UNION OPERATOR WITH PLUS
    ("xutri", "\u{25B3}"), // WHITE UP-POINTING TRIANGLE
    ("xvee", "\u{22C1}"),  // N-ARY LOGICAL OR
    ("xwedge", "\u{22C0}"), // N-ARY LOGICAL AND
    ("yacute", "\u{00FD}"), // LATIN SMALL LETTER Y WITH ACUTE
    ("yacy", "\u{044F}"),  // CYRILLIC SMALL LETTER YA
    ("ycirc", "\u{0177}"), // LATIN SMALL LETTER Y WITH CIRCUMFLEX
    ("ycy", "\u{044B}"),   // CYRILLIC SMALL LETTER YERU
    ("yen", "\u{00A5}"),   // YEN SIGN
    ("yfr", "\u{1D536}"),  // MATHEMATICAL FRAKTUR SMALL Y
    ("yicy", "\u{0457}"),  // CYRILLIC SMALL LETTER YI
    ("yopf", "\u{1D56A}"), // MATHEMATICAL DOUBLE-STRUCK SMALL Y
    ("yscr", "\u{1D4CE}"), // MATHEMATICAL SCRIPT SMALL Y
    ("yucy", "\u{044E}"),  // CYRILLIC SMALL LETTER YU
    ("yuml", "\u{00FF}"),  // LATIN SMALL LETTER Y WITH DIAERESIS
    ("zacute", "\u{017A}"), // LATIN SMALL LETTER Z WITH ACUTE
    ("zcaron", "\u{017E}"), // LATIN SMALL LETTER Z WITH CARON
    ("zcy", "\u{0437}"),   // CYRILLIC SMALL LETTER ZE
    ("zdot", "\u{017C}"),  // LATIN SMALL LETTER Z WITH DOT ABOVE
    ("zeta", "\u{03B6}"),  // GREEK SMALL LETTER ZETA
    ("zfr", "\u{1D537}"),  // MATHEMATICAL FRAKTUR SMALL Z
    ("zhcy", "\u{0436}"),  // CYRILLIC SMALL LETTER ZHE
    ("zigrarr", "\u{21DD}"), // RIGHTWARDS SQUIGGLE ARROW
    ("zopf", "\u{1D56B}"), // MATHEMATICAL DOUBLE-STRUCK SMALL Z
    ("zscr", "\u{1D4CF}"), // MATHEMATICAL SCRIPT SMALL Z
    ("zwj", "\u{200D}"),   // ZERO WIDTH JOINER
    ("zwnj", "\u{200C}"),  // ZERO WIDTH NON-JOINER
];
