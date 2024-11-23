#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Cabin internals -----------------------------------------------------------------------------------------------------------------

typedef struct {
	char* value;
	int capacity;
} DynamicString;

void push_to_dynamic_string(DynamicString* string, char* append) {
	if (strlen(string->value) + strlen(append) > string->capacity) {
		string->capacity *= 2;
		string->value = (char*) realloc(string->value, sizeof(char) * string->capacity);
	}

	strcat(&string->value, append);
}

// Forward declarations -----------------------------------------------------------------------

typedef enum either_u_Boolean_106 u_Boolean_106;
typedef struct group_u_BuiltinTag_31 group_u_BuiltinTag_31;
typedef struct group_u_Object_33 group_u_Object_33;
typedef enum either_u_Nothing_14 u_Nothing_14;
typedef struct type_u_anonymous_object_103 type_u_anonymous_object_103;
typedef struct group_u_Group_27 group_u_Group_27;
typedef struct group_u_This_8 group_u_This_8;
typedef struct group_u_Parameter_61 group_u_Parameter_61;
typedef struct group_u_Either_95 group_u_Either_95;
typedef struct group_u_Error_133 group_u_Error_133;
typedef struct group_u_Text_20 group_u_Text_20;
typedef struct group_u_OneOf_68 group_u_OneOf_68;
typedef struct type_u_anonymous_object_11 type_u_anonymous_object_11;
typedef struct type_u_terminal_153 type_u_terminal_153;
typedef struct type_u_system_side_effects_83 type_u_system_side_effects_83;
typedef struct group_u_Number_108 group_u_Number_108;
typedef struct group_u_Field_85 group_u_Field_85;
typedef struct type_u_cabin_only_34 type_u_cabin_only_34;
typedef struct group_u_List_92 group_u_List_92;
typedef struct group_u_Function_71 group_u_Function_71;
typedef struct group_u_Anything_23 group_u_Anything_23;
typedef struct type_u_anonymous_object_100 type_u_anonymous_object_100;

enum u_Boolean {

	u_true,
	u_false,
};

struct group_u_BuiltinTag_31 {
	group_u_Function_71* u_type;
	group_u_Function_71* u_to_string;
	group_u_Object_33* u_internal_name;
};

struct group_u_Object_33 {
	group_u_Function_71* u_type;
	group_u_Function_71* u_to_string;
};

enum u_Nothing {

	u_nothing,
};

struct type_u_anonymous_object_103 {
	group_u_Function_71* u_type;
	group_u_Function_71* u_to_string;
};

struct group_u_Group_27 {
	group_u_Function_71* u_type;
	group_u_Function_71* u_to_string;
	group_u_Object_33* u_fields;
};

struct group_u_This_8 {
	group_u_Function_71* u_type;
	group_u_Function_71* u_to_string;
};

struct group_u_Parameter_61 {
	group_u_Function_71* u_type;
	group_u_Function_71* u_to_string;
	group_u_Object_33* u_name;
	group_u_Object_33* u_type;
};

struct group_u_Either_95 {
	group_u_Function_71* u_type;
	group_u_Function_71* u_to_string;
	group_u_Object_33* u_variants;
};

struct group_u_Error_133 {
	group_u_Function_71* u_type;
	group_u_Function_71* u_to_string;
	group_u_Object_33* u_message;
};

struct group_u_Text_20 {
	group_u_Function_71* u_type;
	group_u_Function_71* u_to_string;
	char* internal_value;
};

struct group_u_OneOf_68 {
	group_u_Function_71* u_type;
	group_u_Function_71* u_to_string;
};

struct type_u_anonymous_object_11 {
	group_u_Function_71* u_type;
	group_u_Function_71* u_to_string;
};

struct type_u_terminal_153 {
	group_u_Function_71* u_type;
	group_u_Function_71* u_to_string;
	group_u_Function_71* u_print;
	group_u_Function_71* u_to_string;
	group_u_Function_71* u_type;
	group_u_Function_71* u_input;
};

struct type_u_system_side_effects_83 {
	group_u_Function_71* u_type;
	group_u_Function_71* u_to_string;
	group_u_Function_71* u_type;
	group_u_Function_71* u_to_string;
};

struct group_u_Number_108 {
	group_u_Function_71* u_type;
	group_u_Function_71* u_to_string;
	group_u_Function_71* u_plus;
	group_u_Function_71* u_minus;
	float internal_value;
};

struct group_u_Field_85 {
	group_u_Function_71* u_type;
	group_u_Function_71* u_to_string;
	group_u_Object_33* u_name;
	group_u_Object_33* u_value;
};

struct type_u_cabin_only_34 {
	group_u_Function_71* u_type;
	group_u_Function_71* u_to_string;
};

struct group_u_List_92 {
	group_u_Function_71* u_type;
	group_u_Function_71* u_to_string;
	void* elements;
	int size;
	int capacity;
};

struct group_u_Function_71 {
	group_u_Function_71* u_type;
	group_u_Function_71* u_to_string;
	group_u_Object_33* u_parameters;
	group_u_Object_33* u_return_type;
	group_u_Object_33* u_compile_time_parameters;
	group_u_Object_33* u_tags;
	group_u_Object_33* u_this_object;
	void* call;
};

struct group_u_Anything_23 {
	group_u_Function_71* u_type;
	group_u_Function_71* u_to_string;
};

struct type_u_anonymous_object_100 {
	group_u_Function_71* u_type;
	group_u_Function_71* u_to_string;
};

void call_anonymous_function_126(group_u_Number_108* u_this, group_u_Number_108* u_other, group_u_Number_108* return_address) {

}

void call_anonymous_function_159(group_u_Anything_23* u_object) {
	group_u_Text_20* return_address;
	(((void (*)(group_u_Anything_23*, group_u_Text_20*))(u_object->u_to_string->call))(u_object, return_address));
	printf("%s\n", return_address->internal_value);
}

void call_anonymous_function_54(group_u_Anything_23* u_this, group_u_Text_20* return_address) {
	// Get the type metadata of the value
	group_u_Group_27 type;
	(((void (*)(group_u_Anything_23*, group_u_Group_27*))(u_this->u_type->call))(u_this, &type));
	
	// Build the string
	DynamicString result = (DynamicString) { .value = type.name, .capacity = 16 };
	push_to_dynamic_string(&result, " {");
	
	// Add fields
	for (int fieldNumber = 0; fieldNumber < type.u_fields->elements.size; fieldNumber++) {										
		push_to_dynamic_string(&result, (char*) type.u_fields->elements.data[fieldNumber]);
	}
	
	// Return the built string
	push_to_dynamic_string(&result, " }");
	*return_address = (group_u_Text_20) { .internal_value = result.value };
}

void call_anonymous_function_117(group_u_Number_108* u_this, group_u_Number_108* u_other, group_u_Number_108* return_address) {
	*return_address = (group_u_Number_108) { .internal_value = u_this->internal_value + u_other->internal_value };
}

void call_anonymous_function_147(group_u_Anything_23* u_object) {
	group_u_Text_20* return_address;
	(((void (*)(group_u_Anything_23*, group_u_Text_20*))(u_object->u_to_string->call))(u_object, return_address));
	printf("%s\n", return_address->internal_value);
}

void call_anonymous_function_152(group_u_Text_20* return_address) {
	char* buffer = malloc(sizeof(char) * 256);
	fgets(buffer, 256, stdin);
	*return_address = (group_u_Text_20) { .internal_value = buffer };
}

void call_anonymous_function_47(group_u_Anything_23* u_this, group_u_Group_27* return_address) {

}

int main(int argc, char** argv) {
	group_u_List_92* u_anonymous_list_59 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Group_27* u_Anything_23 = &(group_u_Group_27) {
		.u_fields = u_anonymous_list_59,
	};
	
	group_u_List_92* u_anonymous_list_131 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Group_27* u_Number_108 = &(group_u_Group_27) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_fields = u_anonymous_list_131,
	};
	
	group_u_List_92* u_anonymous_list_125 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_List_92* u_anonymous_list_123 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_List_92* u_anonymous_list_124 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Object_33* u_anonymous_object_11 = &(type_u_anonymous_object_11) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Function_71* anonymous_function_126 = &(group_u_Function_71) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_return_type = u_Number_108,
		.u_tags = u_anonymous_list_125,
		.u_compile_time_parameters = u_anonymous_list_123,
		.u_parameters = u_anonymous_list_124,
		.u_this_object = u_anonymous_object_11,
		.call = &call_anonymous_function_126
	};
	
	group_u_Text_20* anonymous_string_literal_129 = &(group_u_Text_20) { .internal_value = "minus" };
	
	group_u_Field_85* u_Number_minus_130 = &(group_u_Field_85) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_value = anonymous_function_126,
		.u_name = anonymous_string_literal_129,
	};
	
	group_u_Text_20* anonymous_string_literal_134 = &(group_u_Text_20) { .internal_value = "message" };
	
	group_u_List_92* u_anonymous_list_21 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_List_92* u_anonymous_list_150 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Text_20* anonymous_string_literal_16 = &(group_u_Text_20) { .internal_value = "Data" };
	
	group_u_List_92* u_anonymous_list_30 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Group_27* u_BuiltinTag_31 = &(group_u_Group_27) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_fields = u_anonymous_list_30,
	};
	
	group_u_List_92* u_anonymous_list_32 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Group_27* u_Object_33 = &(group_u_Group_27) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_fields = u_anonymous_list_32,
	};
	
	group_u_Text_20* anonymous_string_literal_78 = &(group_u_Text_20) { .internal_value = "tags" };
	
	group_u_Text_20* anonymous_string_literal_86 = &(group_u_Text_20) { .internal_value = "name" };
	
	group_u_Field_85* u_Field_name_87 = &(group_u_Field_85) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_name = anonymous_string_literal_86,
		.u_value = u_anonymous_object_11,
	};
	
	group_u_List_92* u_anonymous_list_132 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_List_92* u_anonymous_list_139 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_List_92* u_anonymous_list_146 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Text_20* anonymous_string_literal_74 = &(group_u_Text_20) { .internal_value = "return_type" };
	
	group_u_Text_20* anonymous_string_literal_102 = &(group_u_Text_20) { .internal_value = "false" };
	
	group_u_List_92* u_anonymous_list_60 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Text_20* anonymous_string_literal_127 = &(group_u_Text_20) { .internal_value = "plus" };
	
	group_u_List_92* u_anonymous_list_116 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_List_92* u_anonymous_list_115 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_List_92* u_anonymous_list_114 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Function_71* anonymous_function_117 = &(group_u_Function_71) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_tags = u_anonymous_list_116,
		.u_parameters = u_anonymous_list_115,
		.u_compile_time_parameters = u_anonymous_list_114,
		.u_this_object = u_anonymous_object_11,
		.u_return_type = u_Number_108,
		.call = &call_anonymous_function_117
	};
	
	group_u_Field_85* u_Number_plus_128 = &(group_u_Field_85) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_name = anonymous_string_literal_127,
		.u_value = anonymous_function_117,
	};
	
	group_u_List_92* u_anonymous_list_53 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_List_92* u_anonymous_list_13 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_List_92* u_anonymous_list_151 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Text_20* anonymous_string_literal_112 = &(group_u_Text_20) { .internal_value = "other" };
	
	group_u_Parameter_61* u_anonymous_function_other_113 = &(group_u_Parameter_61) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_name = anonymous_string_literal_112,
		.u_type = u_Number_108,
	};
	
	group_u_Text_20* anonymous_string_literal_88 = &(group_u_Text_20) { .internal_value = "value" };
	
	group_u_Field_85* u_Field_value_89 = &(group_u_Field_85) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_name = anonymous_string_literal_88,
		.u_value = u_anonymous_object_11,
	};
	
	group_u_List_92* u_anonymous_list_93 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Text_20* anonymous_string_literal_119 = &(group_u_Text_20) { .internal_value = "this" };
	
	group_u_Parameter_61* u_anonymous_function_this_120 = &(group_u_Parameter_61) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_name = anonymous_string_literal_119,
		.u_type = u_Number_108,
	};
	
	group_u_Text_20* anonymous_string_literal_35 = &(group_u_Text_20) { .internal_value = "name" };
	
	group_u_Text_20* anonymous_string_literal_24 = &(group_u_Text_20) { .internal_value = "fields" };
	
	group_u_List_92* u_anonymous_list_52 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Group_27* u_Text_20 = &(group_u_Group_27) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_fields = u_anonymous_list_21,
	};
	
	group_u_List_92* u_anonymous_list_51 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Function_71* anonymous_function_54 = &(group_u_Function_71) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_parameters = u_anonymous_list_52,
		.u_tags = u_anonymous_list_53,
		.u_this_object = u_anonymous_object_11,
		.u_return_type = u_Text_20,
		.u_compile_time_parameters = u_anonymous_list_51,
		.call = &call_anonymous_function_54
	};
	
	group_u_List_92* u_anonymous_list_46 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_List_92* u_anonymous_list_26 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Group_27* u_Group_27 = &(group_u_Group_27) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_fields = u_anonymous_list_26,
	};
	
	group_u_List_92* u_anonymous_list_45 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_List_92* u_anonymous_list_44 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Function_71* anonymous_function_47 = &(group_u_Function_71) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_tags = u_anonymous_list_46,
		.u_return_type = u_Group_27,
		.u_parameters = u_anonymous_list_45,
		.u_compile_time_parameters = u_anonymous_list_44,
		.u_this_object = u_anonymous_object_11,
		.call = &call_anonymous_function_47
	};
	
	group_u_Text_20* anonymous_string_literal_3 = &(group_u_Text_20) { .internal_value = "Number.minus" };
	
	group_u_BuiltinTag_31* anonymous_object_118 = &(group_u_BuiltinTag_31) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_to_string = anonymous_function_54,
		.u_type = anonymous_function_47,
		.u_internal_name = anonymous_string_literal_3,
	};
	
	group_u_List_92* u_anonymous_list_91 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Field_85* u_Function_return_type_75 = &(group_u_Field_85) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_name = anonymous_string_literal_74,
		.u_value = u_anonymous_object_11,
	};
	
	group_u_Text_20* anonymous_string_literal_76 = &(group_u_Text_20) { .internal_value = "compile_time_parameters" };
	
	group_u_Object_33* u_anonymous_object_103 = &(type_u_anonymous_object_103) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Text_20* anonymous_string_literal_64 = &(group_u_Text_20) { .internal_value = "type" };
	
	group_u_List_92* u_anonymous_list_137 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Text_20* anonymous_string_literal_154 = &(group_u_Text_20) { .internal_value = "object" };
	
	group_u_Field_85* u_Parameter_type_65 = &(group_u_Field_85) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_name = anonymous_string_literal_64,
		.u_value = u_anonymous_object_11,
	};
	
	group_u_Text_20* anonymous_string_literal_4 = &(group_u_Text_20) { .internal_value = "terminal.print" };
	
	group_u_List_92* u_anonymous_list_70 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Field_85* u_Function_compile_time_parameters_77 = &(group_u_Field_85) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_value = u_anonymous_object_11,
		.u_name = anonymous_string_literal_76,
	};
	
	group_u_Text_20* anonymous_string_literal_72 = &(group_u_Text_20) { .internal_value = "parameters" };
	
	group_u_List_92* u_anonymous_list_9 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Group_27* u_This_8 = &(group_u_Group_27) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_fields = u_anonymous_list_9,
	};
	
	group_u_Text_20* anonymous_string_literal_10 = &(group_u_Text_20) { .internal_value = "nothing" };
	
	group_u_List_92* u_anonymous_list_136 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_List_92* u_anonymous_list_157 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Text_20* anonymous_string_literal_55 = &(group_u_Text_20) { .internal_value = "type" };
	
	group_u_Text_20* anonymous_string_literal_5 = &(group_u_Text_20) { .internal_value = "terminal.input" };
	
	group_u_Text_20* anonymous_string_literal_99 = &(group_u_Text_20) { .internal_value = "true" };
	
	group_u_Object_33* u_anonymous_object_100 = &(type_u_anonymous_object_100) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Field_85* u_true_101 = &(group_u_Field_85) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_name = anonymous_string_literal_99,
		.u_value = u_anonymous_object_100,
	};
	
	group_u_List_92* u_anonymous_list_66 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Group_27* u_Parameter_61 = &(group_u_Group_27) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_fields = u_anonymous_list_66,
	};
	
	group_u_List_92* u_anonymous_list_149 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_List_92* u_anonymous_list_156 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Text_20* anonymous_string_literal_110 = &(group_u_Text_20) { .internal_value = "this" };
	
	group_u_Parameter_61* u_anonymous_function_this_111 = &(group_u_Parameter_61) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_type = u_Number_108,
		.u_name = anonymous_string_literal_110,
	};
	
	group_u_List_92* u_anonymous_list_98 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Group_27* u_Either_95 = &(group_u_Group_27) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_fields = u_anonymous_list_98,
	};
	
	group_u_List_92* u_anonymous_list_22 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Group_27* u_Error_133 = &(group_u_Group_27) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_fields = u_anonymous_list_136,
	};
	
	group_u_List_92* u_anonymous_list_37 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Text_20* anonymous_string_literal_0 = &(group_u_Text_20) { .internal_value = "Anything.type" };
	
	group_u_BuiltinTag_31* anonymous_object_41 = &(group_u_BuiltinTag_31) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_internal_name = anonymous_string_literal_0,
	};
	
	group_u_Text_20* anonymous_string_literal_121 = &(group_u_Text_20) { .internal_value = "other" };
	
	group_u_Text_20* anonymous_string_literal_57 = &(group_u_Text_20) { .internal_value = "to_string" };
	
	group_u_Field_85* u_Anything_to_string_58 = &(group_u_Field_85) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_name = anonymous_string_literal_57,
		.u_value = anonymous_function_54,
	};
	
	group_u_Text_20* anonymous_string_literal_62 = &(group_u_Text_20) { .internal_value = "name" };
	
	group_u_Field_85* u_Parameter_name_63 = &(group_u_Field_85) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_name = anonymous_string_literal_62,
		.u_value = u_anonymous_object_11,
	};
	
	group_u_List_92* u_anonymous_list_84 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_List_92* u_anonymous_list_69 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Group_27* u_OneOf_68 = &(group_u_Group_27) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_fields = u_anonymous_list_69,
	};
	
	group_u_List_92* u_anonymous_list_19 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_List_92* u_anonymous_list_158 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_List_92* u_anonymous_list_144 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_List_92* u_anonymous_list_145 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Function_71* anonymous_function_147 = &(group_u_Function_71) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_tags = u_anonymous_list_146,
		.u_compile_time_parameters = u_anonymous_list_144,
		.u_this_object = u_anonymous_object_11,
		.u_return_type = u_anonymous_object_11,
		.u_parameters = u_anonymous_list_145,
		.call = &call_anonymous_function_147
	};
	
	group_u_Function_71* anonymous_function_152 = &(group_u_Function_71) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_return_type = u_Text_20,
		.u_parameters = u_anonymous_list_150,
		.u_compile_time_parameters = u_anonymous_list_149,
		.u_tags = u_anonymous_list_151,
		.u_this_object = u_anonymous_object_11,
		.call = &call_anonymous_function_152
	};
	
	group_u_Object_33* u_terminal_153 = &(type_u_terminal_153) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_print = anonymous_function_147,
		.u_to_string = anonymous_function_54,
		.u_type = anonymous_function_47,
		.u_input = anonymous_function_152,
	};
	
	group_u_Function_71* anonymous_function_159 = &(group_u_Function_71) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_return_type = u_anonymous_object_11,
		.u_compile_time_parameters = u_anonymous_list_156,
		.u_tags = u_anonymous_list_158,
		.u_parameters = u_anonymous_list_157,
		.u_this_object = u_terminal_153,
		.call = &call_anonymous_function_159
	};
	
	group_u_Field_85* u_false_104 = &(group_u_Field_85) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_name = anonymous_string_literal_102,
		.u_value = u_anonymous_object_103,
	};
	
	group_u_Text_20* anonymous_string_literal_80 = &(group_u_Text_20) { .internal_value = "this_object" };
	
	group_u_Field_85* u_Function_this_object_81 = &(group_u_Field_85) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_name = anonymous_string_literal_80,
		.u_value = u_anonymous_object_11,
	};
	
	group_u_Parameter_61* u_anonymous_function_other_122 = &(group_u_Parameter_61) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_type = u_Number_108,
		.u_name = anonymous_string_literal_121,
	};
	
	group_u_Text_20* anonymous_string_literal_1 = &(group_u_Text_20) { .internal_value = "Anything.to_string" };
	
	group_u_List_92* u_anonymous_list_107 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_BuiltinTag_31* anonymous_object_48 = &(group_u_BuiltinTag_31) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_internal_name = anonymous_string_literal_1,
	};
	
	group_u_Field_85* u_Anything_type_56 = &(group_u_Field_85) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_value = anonymous_function_47,
		.u_name = anonymous_string_literal_55,
	};
	
	group_u_List_92* u_anonymous_list_90 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_BuiltinTag_31* anonymous_object_148 = &(group_u_BuiltinTag_31) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_to_string = anonymous_function_54,
		.u_internal_name = anonymous_string_literal_5,
		.u_type = anonymous_function_47,
	};
	
	group_u_Field_85* u_Function_tags_79 = &(group_u_Field_85) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_value = u_anonymous_object_11,
		.u_name = anonymous_string_literal_78,
	};
	
	group_u_Text_20* anonymous_string_literal_2 = &(group_u_Text_20) { .internal_value = "Number.plus" };
	
	group_u_Text_20* anonymous_string_literal_142 = &(group_u_Text_20) { .internal_value = "object" };
	
	group_u_Text_20* anonymous_string_literal_138 = &(group_u_Text_20) { .internal_value = "Data" };
	
	group_u_List_92* u_anonymous_list_105 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_List_92* u_anonymous_list_38 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Text_20* anonymous_string_literal_42 = &(group_u_Text_20) { .internal_value = "this" };
	
	group_u_Text_20* anonymous_string_literal_96 = &(group_u_Text_20) { .internal_value = "variants" };
	
	group_u_BuiltinTag_31* anonymous_object_141 = &(group_u_BuiltinTag_31) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_to_string = anonymous_function_54,
		.u_type = anonymous_function_47,
		.u_internal_name = anonymous_string_literal_4,
	};
	
	group_u_Field_85* u_nothing_12 = &(group_u_Field_85) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_name = anonymous_string_literal_10,
		.u_value = u_anonymous_object_11,
	};
	
	group_u_List_92* u_anonymous_list_39 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_List_92* u_anonymous_list_17 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Object_33* u_system_side_effects_83 = &(type_u_system_side_effects_83) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Parameter_61* u_anonymous_function_this_43 = &(group_u_Parameter_61) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_name = anonymous_string_literal_42,
		.u_type = u_Anything_23,
	};
	
	group_u_List_92* u_anonymous_list_94 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_List_92* u_anonymous_list_82 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Field_85* u_Error_message_135 = &(group_u_Field_85) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_name = anonymous_string_literal_134,
		.u_value = u_anonymous_object_11,
	};
	
	group_u_Parameter_61* u_builtin_name_36 = &(group_u_Parameter_61) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_name = anonymous_string_literal_35,
		.u_type = u_Text_20,
	};
	
	group_u_Parameter_61* u_anonymous_function_object_143 = &(group_u_Parameter_61) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_type = u_Anything_23,
		.u_name = anonymous_string_literal_142,
	};
	
	group_u_Text_20* anonymous_string_literal_49 = &(group_u_Text_20) { .internal_value = "this" };
	
	group_u_Parameter_61* u_anonymous_function_this_50 = &(group_u_Parameter_61) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_type = u_Anything_23,
		.u_name = anonymous_string_literal_49,
	};
	
	group_u_List_92* u_anonymous_list_15 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_List_92* u_anonymous_list_7 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Text_20* anonymous_string_literal_28 = &(group_u_Text_20) { .internal_value = "internal_name" };
	
	group_u_Parameter_61* u_anonymous_function_object_155 = &(group_u_Parameter_61) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_name = anonymous_string_literal_154,
		.u_type = u_Anything_23,
	};
	
	group_u_Group_27* u_Field_85 = &(group_u_Group_27) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_fields = u_anonymous_list_90,
	};
	
	group_u_Field_85* u_Either_variants_97 = &(group_u_Field_85) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_value = u_anonymous_object_11,
		.u_name = anonymous_string_literal_96,
	};
	
	group_u_Object_33* u_cabin_only_34 = &(type_u_cabin_only_34) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Field_85* u_Function_parameters_73 = &(group_u_Field_85) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_name = anonymous_string_literal_72,
		.u_value = u_anonymous_object_11,
	};
	
	group_u_BuiltinTag_31* anonymous_object_109 = &(group_u_BuiltinTag_31) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_internal_name = anonymous_string_literal_2,
	};
	
	group_u_Group_27* u_List_92 = &(group_u_Group_27) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_fields = u_anonymous_list_93,
	};
	
	group_u_Text_20* anonymous_string_literal_6 = &(group_u_Text_20) { .internal_value = "Hello world!" };
	
	group_u_Field_85* u_Group_fields_25 = &(group_u_Field_85) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_name = anonymous_string_literal_24,
		.u_value = u_anonymous_object_11,
	};
	
	group_u_List_92* u_anonymous_list_67 = &(group_u_List_92) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
	};
	
	group_u_Group_27* u_Function_71 = &(group_u_Group_27) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_fields = u_anonymous_list_82,
	};
	
	group_u_Field_85* u_BuiltinTag_internal_name_29 = &(group_u_Field_85) {
		.u_type = anonymous_function_47,
		.u_to_string = anonymous_function_54,
		.u_value = u_anonymous_object_11,
		.u_name = anonymous_string_literal_28,
	};
	
	({
			
		group_u_Text_20* arg0 = anonymous_string_literal_6;
		(((void (*)(group_u_Anything_23*))(anonymous_function_159->call))(arg0));
		
	})	
	;

	return 0;
}