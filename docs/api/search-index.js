var searchIndex = JSON.parse('{\
"cliparser":{"doc":"cliparser","t":[5,5,5,5,5,0,5,3,4,4,4,3,3,3,4,13,13,13,13,13,13,13,13,4,3,13,13,13,13,13,12,12,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,11,11,11,11,11,11,11,11,11,11,12,12,12,12,11,11,11,11,11,11,11,11,11,11,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,12,12,12,12],"n":["help","parse","parse_any","parse_process","parse_process_any","types","version","Argument","ArgumentHelp","ArgumentOccurrence","ArgumentValueType","CliParsed","CliSpec","CliSpecMetaInfo","Command","Command","CommandDoesNotMatchSpec","InternalError","InvalidCliSpec","InvalidCommandLine","Multiple","Multiple","None","ParserError","PositionalArgument","Single","Single","SubCommand","Text","TextAndParam","argument_occurrence","argument_values","arguments","arguments","author","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone","clone","clone","clone","clone","clone","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","clone_into","command","default","default","default","default_value","description","eq","eq","eq","eq","eq","eq","eq","eq","eq","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from","from","from","from","help","help","help_post_text","into","into","into","into","into","into","into","into","into","into","key","meta_info","name","name","ne","ne","ne","ne","ne","ne","ne","new","new","new","positional_argument","project","source","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_owned","to_string","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","type_id","value_type","version","0","0","1","0","0","0","0","0","0"],"q":["cliparser","","","","","","","cliparser::types","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","cliparser::types::ArgumentHelp","","","cliparser::types::Command","","cliparser::types::ParserError","","",""],"d":["Generates and returns the spec help text","Parsers the given command line based on the given spec and …","Parsers the given command line based on the given specs …","Parsers the given command line based on the given spec and …","Parsers the current process command line based on the …","types","Generates and returns the spec version text","Holds the command line argument spec","The argument help text","The argument occurrence type (see values for more info)","The argument value type (see values for more info)","Holds the command line parse result","Holds the command line spec (command/parameters/…)","Holds the command line spec meta information used to …","The command (not params) string/s","Single command (not sub command) such as: “ls”. Any …","Error Info Type","Error Info Type","Error Info Type","Error Info Type","The argument can appear multiple times. The value of each …","Allows multiple values (minimum one)","The argument does not accept any value","Holds the error information","Holds the positional argument spec","The argument can appear only once","Only single value is allowed","Sub command value such as: vec![“cargo”.to_string(), …","Text value","Text and variable name","The argument occurrence (see enum)","A map of all values for arguments found. The map will …","A list of all possible command line arguments.","A collection of all arguments found (list of names not …","Author name","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","A list of all possible commands and sub commands.","","","","Default value if not found","Description string","","","","","","","","","","","","","","","","","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Help text","Help text","Post help text","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","All possible argument keys in the command line (for …","Meta information used for generating version and help …","Unique name for the argument later used to pull the parsed …","Unique name for the argument later used to pull the parsed …","","","","","","","","Returns new instance","Returns new instance","Returns new instance","The name of the argument that will hold all arguments …","Project name","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","The possible value type for this specific argument","Version string","","","","","","","","",""],"i":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,2,2,2,2,3,4,4,0,0,3,4,1,5,5,6,7,8,7,9,2,1,3,4,5,6,10,9,8,7,2,1,3,4,5,6,10,9,8,7,1,3,4,5,6,10,9,8,7,1,3,4,5,6,10,9,8,7,8,9,8,7,6,9,1,3,4,5,6,10,9,8,7,2,2,1,3,4,5,6,10,9,8,7,2,1,3,4,5,6,10,9,8,7,6,10,9,2,1,3,4,5,6,10,9,8,7,6,8,6,10,1,5,6,10,9,8,7,9,8,7,8,9,2,1,3,4,5,6,10,9,8,7,2,2,1,3,4,5,6,10,9,8,7,2,1,3,4,5,6,10,9,8,7,2,1,3,4,5,6,10,9,8,7,6,9,11,12,12,13,14,15,16,17,18],"f":[[[["clispec",3]],["string",3]],[[["vec",3],["clispec",3]],["result",4,[["cliparsed",3],["parsererror",4]]]],[[["vec",3],["vec",3,[["clispec",3]]]],["result",4,[["parsererror",4]]]],[[["clispec",3]],["result",4,[["cliparsed",3],["parsererror",4]]]],[[["vec",3,[["clispec",3]]]],["result",4,[["parsererror",4]]]],null,[[["clispec",3]],["string",3]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["command",4]],["command",4]],[[["argumentoccurrence",4]],["argumentoccurrence",4]],[[["argumentvaluetype",4]],["argumentvaluetype",4]],[[["argumenthelp",4]],["argumenthelp",4]],[[["argument",3]],["argument",3]],[[["positionalargument",3]],["positionalargument",3]],[[["clispecmetainfo",3]],["clispecmetainfo",3]],[[["clispec",3]],["clispec",3]],[[["cliparsed",3]],["cliparsed",3]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],null,[[],["clispecmetainfo",3]],[[],["clispec",3]],[[],["cliparsed",3]],null,null,[[["command",4],["command",4]],["bool",0]],[[["argumentoccurrence",4],["argumentoccurrence",4]],["bool",0]],[[["argumentvaluetype",4],["argumentvaluetype",4]],["bool",0]],[[["argumenthelp",4],["argumenthelp",4]],["bool",0]],[[["argument",3],["argument",3]],["bool",0]],[[["positionalargument",3],["positionalargument",3]],["bool",0]],[[["clispecmetainfo",3],["clispecmetainfo",3]],["bool",0]],[[["clispec",3],["clispec",3]],["bool",0]],[[["cliparsed",3],["cliparsed",3]],["bool",0]],[[["parsererror",4],["formatter",3]],["result",6]],[[["parsererror",4],["formatter",3]],["result",4,[["error",3]]]],[[["command",4],["formatter",3]],["result",6]],[[["argumentoccurrence",4],["formatter",3]],["result",6]],[[["argumentvaluetype",4],["formatter",3]],["result",6]],[[["argumenthelp",4],["formatter",3]],["result",6]],[[["argument",3],["formatter",3]],["result",6]],[[["positionalargument",3],["formatter",3]],["result",6]],[[["clispecmetainfo",3],["formatter",3]],["result",6]],[[["clispec",3],["formatter",3]],["result",6]],[[["cliparsed",3],["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,null,null,null,[[["command",4],["command",4]],["bool",0]],[[["argumenthelp",4],["argumenthelp",4]],["bool",0]],[[["argument",3],["argument",3]],["bool",0]],[[["positionalargument",3],["positionalargument",3]],["bool",0]],[[["clispecmetainfo",3],["clispecmetainfo",3]],["bool",0]],[[["clispec",3],["clispec",3]],["bool",0]],[[["cliparsed",3],["cliparsed",3]],["bool",0]],[[],["clispecmetainfo",3]],[[],["clispec",3]],[[],["cliparsed",3]],null,null,[[["parsererror",4]],["option",4,[["error",8]]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],null,null,null,null,null,null,null,null,null,null,null],"p":[[4,"Command"],[4,"ParserError"],[4,"ArgumentOccurrence"],[4,"ArgumentValueType"],[4,"ArgumentHelp"],[3,"Argument"],[3,"CliParsed"],[3,"CliSpec"],[3,"CliSpecMetaInfo"],[3,"PositionalArgument"],[13,"Text"],[13,"TextAndParam"],[13,"Command"],[13,"SubCommand"],[13,"InvalidCommandLine"],[13,"InvalidCliSpec"],[13,"CommandDoesNotMatchSpec"],[13,"InternalError"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};
