#include <stdio.h>
#include <stdlib.h>

typedef struct u_BuiltinTag u_BuiltinTag;
typedef struct metadata_u_BuiltinTag metadata_u_BuiltinTag;
typedef struct type_u_cabin_only type_u_cabin_only;
typedef struct u_Parameter u_Parameter;
typedef struct metadata_u_Parameter metadata_u_Parameter;
typedef struct u_Either u_Either;
typedef struct metadata_u_Either metadata_u_Either;
typedef struct type_POINTER_62 type_POINTER_62;
typedef struct type_u_system_side_effects type_u_system_side_effects;
typedef struct u_This u_This;
typedef struct metadata_u_This metadata_u_This;
typedef struct u_Field u_Field;
typedef struct metadata_u_Field metadata_u_Field;
typedef struct u_Group u_Group;
typedef struct metadata_u_Group metadata_u_Group;
typedef struct u_Anything u_Anything;
typedef struct metadata_u_Anything metadata_u_Anything;
typedef struct u_List u_List;
typedef struct metadata_u_List metadata_u_List;
typedef struct u_Number u_Number;
typedef struct metadata_u_Number metadata_u_Number;
typedef struct u_Text u_Text;
typedef struct metadata_u_Text metadata_u_Text;
typedef struct type_u_terminal type_u_terminal;
typedef enum POINTER_11 POINTER_11;
typedef enum metadata_POINTER_11 metadata_POINTER_11;
typedef struct u_Error u_Error;
typedef struct metadata_u_Error metadata_u_Error;
typedef struct type_POINTER_59 type_POINTER_59;
typedef struct u_Function u_Function;
typedef struct metadata_u_Function metadata_u_Function;
typedef struct type_POINTER_8 type_POINTER_8;
typedef struct u_Object u_Object;
typedef struct metadata_u_Object metadata_u_Object;
typedef struct u_OneOf u_OneOf;
typedef struct metadata_u_OneOf metadata_u_OneOf;
typedef enum POINTER_65 POINTER_65;
typedef enum metadata_POINTER_65 metadata_POINTER_65;

void anonymous_function_3(u_Anything* u_object) {
	printf("%s", u_object);
}

void anonymous_function_3(u_Anything* u_object) {
	printf("%s", u_object);
}

struct u_BuiltinTag {
	void* u_internal_name;
};

struct metadata_u_BuiltinTag { char empty; };

void anonymous_function_4(u_Text* return_address) {
	char* buffer;
	size_t size;
	getline(&buffer, &size, stdin);
	*return_address = (u_Text) { .internal_value = buffer };
}

struct type_u_cabin_only {
	char empty;
};

struct u_Parameter {
	void* u_name;
	void* u_type;
};

struct metadata_u_Parameter { char empty; };

struct u_Either {
	void* u_variants;
};

struct metadata_u_Either { char empty; };

struct type_POINTER_62 {
	char empty;
};

struct type_u_system_side_effects {
	char empty;
};

struct u_This {
	char empty;
};

struct metadata_u_This { char empty; };

struct u_Field {
	void* u_name;
	void* u_value;
};

struct metadata_u_Field { char empty; };

struct u_Group {
	char empty;
};

struct metadata_u_Group { char empty; };

struct u_Anything {
	char empty;
};

struct metadata_u_Anything { char empty; };

void anonymous_function_1(u_Anything* u_this, u_Anything* u_other, u_Anything* return_address) {
	printf("%s", u_this);
}

struct u_List {
	char empty;
};

struct metadata_u_List { char empty; };

struct u_Number {
	void* u_plus;
	void* u_minus;
};

struct metadata_u_Number { char empty; };

struct u_Text {
	char* internal_value;
};

struct metadata_u_Text { char empty; };

struct type_u_terminal {
	u_Function u_print;
	u_Function u_input;
};

enum POINTER_11 {

	u_nothing,
};



struct u_Error {
	void* u_message;
};

struct metadata_u_Error { char empty; };

struct type_POINTER_59 {
	char empty;
};

struct u_Function {
	void* u_parameters;
	void* u_return_type;
	void* u_compile_time_parameters;
	void* u_tags;
	void* u_this_object;
};

struct metadata_u_Function { char empty; };

struct type_POINTER_8 {
	char empty;
};

void anonymous_function_2(u_Anything* u_this, u_Anything* u_other, u_Anything* return_address) {
	printf("%s", u_this);
}

struct u_Object {
	char empty;
};

struct metadata_u_Object { char empty; };

struct u_OneOf {
	char empty;
};

struct metadata_u_OneOf { char empty; };

enum POINTER_65 {

	u_true,
	u_false,
};



int main(int argc, char** argv) {
	u_Text* anonymous_string_literal_19 = &(u_Text) { .internal_value = "internal_name" };
	
	u_Object* POINTER_8 = &(type_POINTER_8) {
		.empty = '0'
	};
	
	u_Field* u_BuiltinTag_internal_name = &(u_Field) {
		.u_name = anonymous_string_literal_19,
		.u_value = POINTER_8,
	};
	
	u_List* POINTER_125 = &(u_List) {
		.empty = '0'
	};
	
	u_List* POINTER_72 = &(u_List) {
		.empty = '0'
	};
	
	u_Text* anonymous_string_literal_11 = &(u_Text) { .internal_value = "compile_time_parameters" };
	
	u_Field* u_Function_compile_time_parameters = &(u_Field) {
		.u_name = anonymous_string_literal_11,
		.u_value = POINTER_8,
	};
	
	u_List* POINTER_68 = &(u_List) {
		.empty = '0'
	};
	
	u_List* POINTER_30 = &(u_List) {
		.empty = '0'
	};
	
	u_Text* anonymous_string_literal_1 = &(u_Text) { .internal_value = "Number.minus" };
	
	u_BuiltinTag* anonymous_object_2 = &(u_BuiltinTag) {
		.u_internal_name = anonymous_string_literal_1,
	};
	
	u_List* POINTER_18 = &(u_List) {
		.empty = '0'
	};
	
	u_Object* u_cabin_only = &(type_u_cabin_only) {
		.empty = '0'
	};
	
	u_List* POINTER_74 = &(u_List) {
		.empty = '0'
	};
	
	u_Text* anonymous_string_literal_22 = &(u_Text) { .internal_value = "other" };
	
	u_Text* anonymous_string_literal_7 = &(u_Text) { .internal_value = "name" };
	
	u_Text* anonymous_string_literal_31 = &(u_Text) { .internal_value = "Hello world!" };
	
	u_Text* anonymous_string_literal_5 = &(u_Text) { .internal_value = "nothing" };
	
	u_Field* u_Nothing_nothing = &(u_Field) {
		.u_name = anonymous_string_literal_5,
		.u_value = POINTER_8,
	};
	
	u_Text* anonymous_string_literal_13 = &(u_Text) { .internal_value = "this_object" };
	
	u_List* POINTER_5 = &(u_List) {
		.empty = '0'
	};
	
	u_Text* anonymous_string_literal_23 = &(u_Text) { .internal_value = "this" };
	
	u_Object* POINTER_62 = &(type_POINTER_62) {
		.empty = '0'
	};
	
	u_Group* metadata_instance_u_Anything = &(metadata_u_Anything) { .empty = '0' };
	
	u_Text* anonymous_string_literal_29 = &(u_Text) { .internal_value = "object" };
	
	u_Parameter* u_anonymous_function_3_object = &(u_Parameter) {
		.u_type = metadata_instance_u_Anything,
		.u_name = anonymous_string_literal_29,
	};
	
	u_List* POINTER_50 = &(u_List) {
		.empty = '0'
	};
	
	u_Text* anonymous_string_literal_2 = &(u_Text) { .internal_value = "terminal.print" };
	
	u_Text* anonymous_string_literal_17 = &(u_Text) { .internal_value = "true" };
	
	u_List* POINTER_112 = &(u_List) {
		.empty = '0'
	};
	
	u_Text* anonymous_string_literal_24 = &(u_Text) { .internal_value = "other" };
	
	u_Parameter* u_anonymous_function_2_other = &(u_Parameter) {
		.u_name = anonymous_string_literal_24,
		.u_type = metadata_instance_u_Anything,
	};
	
	u_List* POINTER_117 = &(u_List) {
		.empty = '0'
	};
	
	u_Object* u_system_side_effects = &(type_u_system_side_effects) {
		.empty = '0'
	};
	
	u_BuiltinTag* anonymous_object_2 = &(u_BuiltinTag) {
		.u_internal_name = anonymous_string_literal_2,
	};
	
	u_Text* anonymous_string_literal_3 = &(u_Text) { .internal_value = "terminal.input" };
	
	u_BuiltinTag* anonymous_object_2 = &(u_BuiltinTag) {
		.u_internal_name = anonymous_string_literal_3,
	};
	
	u_Text* anonymous_string_literal_18 = &(u_Text) { .internal_value = "false" };
	
	u_Field* u_Boolean_false = &(u_Field) {
		.u_value = POINTER_62,
		.u_name = anonymous_string_literal_18,
	};
	
	u_Text* anonymous_string_literal_10 = &(u_Text) { .internal_value = "return_type" };
	
	u_List* POINTER_81 = &(u_List) {
		.empty = '0'
	};
	
	u_List* POINTER_104 = &(u_List) {
		.empty = '0'
	};
	
	u_Text* anonymous_string_literal_26 = &(u_Text) { .internal_value = "minus" };
	
	u_Field* u_Function_this_object = &(u_Field) {
		.u_value = POINTER_8,
		.u_name = anonymous_string_literal_13,
	};
	
	u_Parameter* u_anonymous_function_2_this = &(u_Parameter) {
		.u_name = anonymous_string_literal_23,
		.u_type = metadata_instance_u_Anything,
	};
	
	u_List* POINTER_14 = &(u_List) {
		.empty = '0'
	};
	
	u_Field* u_Function_return_type = &(u_Field) {
		.u_value = POINTER_8,
		.u_name = anonymous_string_literal_10,
	};
	
	u_Text* anonymous_string_literal_30 = &(u_Text) { .internal_value = "object" };
	
	u_Parameter* u_anonymous_function_3_object = &(u_Parameter) {
		.u_name = anonymous_string_literal_30,
		.u_type = metadata_instance_u_Anything,
	};
	
	u_Text* anonymous_string_literal_12 = &(u_Text) { .internal_value = "tags" };
	
	u_Field* u_Function_tags = &(u_Field) {
		.u_name = anonymous_string_literal_12,
		.u_value = POINTER_8,
	};
	
	u_List* POINTER_82 = &(u_List) {
		.empty = '0'
	};
	
	u_List* POINTER_83 = &(u_List) {
		.empty = '0'
	};
	
	u_List* POINTER_113 = &(u_List) {
		.empty = '0'
	};
	
	u_List* POINTER_111 = &(u_List) {
		.empty = '0'
	};
	
	u_Text* anonymous_string_literal_9 = &(u_Text) { .internal_value = "parameters" };
	
	u_Text* anonymous_string_literal_14 = &(u_Text) { .internal_value = "name" };
	
	u_Field* u_Field_name = &(u_Field) {
		.u_value = POINTER_8,
		.u_name = anonymous_string_literal_14,
	};
	
	u_Function* anonymous_function_3 = &(u_Function) {
		.u_return_type = POINTER_8,
		.u_compile_time_parameters = POINTER_111,
		.u_this_object = POINTER_8,
		.u_parameters = POINTER_112,
		.u_tags = POINTER_113,
	};
	
	u_List* POINTER_116 = &(u_List) {
		.empty = '0'
	};
	
	u_List* POINTER_118 = &(u_List) {
		.empty = '0'
	};
	
	u_List* POINTER_16 = &(u_List) {
		.empty = '0'
	};
	
	u_Group* metadata_instance_u_Text = &(metadata_u_Text) { .empty = '0' };
	
	u_Function* anonymous_function_4 = &(u_Function) {
		.u_compile_time_parameters = POINTER_116,
		.u_tags = POINTER_118,
		.u_return_type = metadata_instance_u_Text,
		.u_this_object = POINTER_8,
		.u_parameters = POINTER_117,
	};
	
	u_Object* u_terminal = &(type_u_terminal) {
		.u_print = anonymous_function_3,
		.u_input = anonymous_function_4,
	};
	
	u_List* POINTER_102 = &(u_List) {
		.empty = '0'
	};
	
	u_Text* anonymous_string_literal_8 = &(u_Text) { .internal_value = "type" };
	
	u_Field* u_Parameter_type = &(u_Field) {
		.u_name = anonymous_string_literal_8,
		.u_value = POINTER_8,
	};
	
	u_List* POINTER_123 = &(u_List) {
		.empty = '0'
	};
	
	u_Text* anonymous_string_literal_27 = &(u_Text) { .internal_value = "message" };
	
	u_List* POINTER_91 = &(u_List) {
		.empty = '0'
	};
	
	u_List* POINTER_90 = &(u_List) {
		.empty = '0'
	};
	
	u_List* POINTER_92 = &(u_List) {
		.empty = '0'
	};
	
	u_Function* anonymous_function_2 = &(u_Function) {
		.u_compile_time_parameters = POINTER_90,
		.u_this_object = POINTER_8,
		.u_return_type = metadata_instance_u_Anything,
		.u_parameters = POINTER_91,
		.u_tags = POINTER_92,
	};
	
	u_Field* u_Number_minus = &(u_Field) {
		.u_name = anonymous_string_literal_26,
		.u_value = anonymous_function_2,
	};
	
	u_Text* anonymous_string_literal_0 = &(u_Text) { .internal_value = "Number.plus" };
	
	u_BuiltinTag* anonymous_object_2 = &(u_BuiltinTag) {
		.u_internal_name = anonymous_string_literal_0,
	};
	
	u_Text* anonymous_string_literal_15 = &(u_Text) { .internal_value = "value" };
	
	u_Field* u_Field_value = &(u_Field) {
		.u_name = anonymous_string_literal_15,
		.u_value = POINTER_8,
	};
	
	u_List* POINTER_124 = &(u_List) {
		.empty = '0'
	};
	
	u_Text* anonymous_string_literal_21 = &(u_Text) { .internal_value = "this" };
	
	u_Parameter* u_anonymous_function_1_this = &(u_Parameter) {
		.u_name = anonymous_string_literal_21,
		.u_type = metadata_instance_u_Anything,
	};
	
	u_Text* anonymous_string_literal_28 = &(u_Text) { .internal_value = "Data" };
	
	u_List* POINTER_56 = &(u_List) {
		.empty = '0'
	};
	
	u_Object* POINTER_59 = &(type_POINTER_59) {
		.empty = '0'
	};
	
	u_Text* anonymous_string_literal_16 = &(u_Text) { .internal_value = "variants" };
	
	u_Field* u_Either_variants = &(u_Field) {
		.u_name = anonymous_string_literal_16,
		.u_value = POINTER_8,
	};
	
	u_Text* anonymous_string_literal_25 = &(u_Text) { .internal_value = "plus" };
	
	u_List* POINTER_20 = &(u_List) {
		.empty = '0'
	};
	
	u_List* POINTER_12 = &(u_List) {
		.empty = '0'
	};
	
	u_List* POINTER_28 = &(u_List) {
		.empty = '0'
	};
	
	u_Field* u_Function_parameters = &(u_Field) {
		.u_value = POINTER_8,
		.u_name = anonymous_string_literal_9,
	};
	
	u_List* POINTER_98 = &(u_List) {
		.empty = '0'
	};
	
	u_List* POINTER_42 = &(u_List) {
		.empty = '0'
	};
	
	u_List* POINTER_73 = &(u_List) {
		.empty = '0'
	};
	
	u_List* POINTER_52 = &(u_List) {
		.empty = '0'
	};
	
	u_Field* u_Parameter_name = &(u_Field) {
		.u_name = anonymous_string_literal_7,
		.u_value = POINTER_8,
	};
	
	u_List* POINTER_10 = &(u_List) {
		.empty = '0'
	};
	
	u_Function* anonymous_function_1 = &(u_Function) {
		.u_compile_time_parameters = POINTER_81,
		.u_tags = POINTER_83,
		.u_this_object = POINTER_8,
		.u_return_type = metadata_instance_u_Anything,
		.u_parameters = POINTER_82,
	};
	
	u_Field* u_Number_plus = &(u_Field) {
		.u_value = anonymous_function_1,
		.u_name = anonymous_string_literal_25,
	};
	
	u_Text* anonymous_string_literal_20 = &(u_Text) { .internal_value = "name" };
	
	u_Text* anonymous_string_literal_6 = &(u_Text) { .internal_value = "Data" };
	
	u_List* POINTER_64 = &(u_List) {
		.empty = '0'
	};
	
	u_Parameter* u_builtin_name = &(u_Parameter) {
		.u_type = metadata_instance_u_Text,
		.u_name = anonymous_string_literal_20,
	};
	
	u_Field* u_Boolean_true = &(u_Field) {
		.u_value = POINTER_59,
		.u_name = anonymous_string_literal_17,
	};
	
	u_Field* u_Error_message = &(u_Field) {
		.u_name = anonymous_string_literal_27,
		.u_value = POINTER_8,
	};
	
	u_List* POINTER_22 = &(u_List) {
		.empty = '0'
	};
	
	u_Text* anonymous_string_literal_4 = &(u_Text) { .internal_value = "Hello world!" };
	
	u_Parameter* u_anonymous_function_1_other = &(u_Parameter) {
		.u_type = metadata_instance_u_Anything,
		.u_name = anonymous_string_literal_22,
	};
	
	u_List* POINTER_106 = &(u_List) {
		.empty = '0'
	};
	
	void;

	return 0;
}