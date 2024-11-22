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

typedef struct group_u_Function_70 group_u_Function_70;
typedef struct group_u_Anything_22 group_u_Anything_22;
typedef struct group_u_Number_107 group_u_Number_107;
typedef struct group_u_OneOf_67 group_u_OneOf_67;
typedef enum either_u_Boolean_105 u_Boolean_105;
typedef enum either_u_Nothing_13 u_Nothing_13;
typedef struct group_u_Text_19 group_u_Text_19;
typedef struct type_u_system_side_effects_82 type_u_system_side_effects_82;
typedef struct group_u_BuiltinTag_30 group_u_BuiltinTag_30;
typedef struct group_u_Field_84 group_u_Field_84;
typedef struct group_u_Error_132 group_u_Error_132;
typedef struct group_u_Group_26 group_u_Group_26;
typedef struct type_u_anonymous_object_99 type_u_anonymous_object_99;
typedef struct group_u_This_7 group_u_This_7;
typedef struct type_u_anonymous_object_102 type_u_anonymous_object_102;
typedef struct group_u_Object_32 group_u_Object_32;
typedef struct type_u_terminal_152 type_u_terminal_152;
typedef struct group_u_Either_94 group_u_Either_94;
typedef struct type_u_anonymous_object_10 type_u_anonymous_object_10;
typedef struct group_u_Parameter_60 group_u_Parameter_60;
typedef struct group_u_List_91 group_u_List_91;
typedef struct type_u_cabin_only_33 type_u_cabin_only_33;

struct group_u_Function_70 {
	group_u_Function_70* u_type;
	group_u_Function_70* u_to_string;
	group_u_Object_32* u_parameters;
	group_u_Object_32* u_return_type;
	group_u_Object_32* u_compile_time_parameters;
	group_u_Object_32* u_tags;
	group_u_Object_32* u_this_object;
	void* call;
};

struct group_u_Anything_22 {
	group_u_Function_70* u_type;
	group_u_Function_70* u_to_string;
};

struct group_u_Number_107 {
	group_u_Function_70* u_type;
	group_u_Function_70* u_to_string;
	group_u_Function_70* u_plus;
	group_u_Function_70* u_minus;
	float internal_value;
};

struct group_u_OneOf_67 {
	group_u_Function_70* u_type;
	group_u_Function_70* u_to_string;
};

enum u_Boolean {

	u_true,
	u_false,
};

enum u_Nothing {

	u_nothing,
};

struct group_u_Text_19 {
	group_u_Function_70* u_type;
	group_u_Function_70* u_to_string;
	char* internal_value;
};

struct type_u_system_side_effects_82 {
	group_u_Function_70* u_type;
	group_u_Function_70* u_to_string;
	group_u_Function_70* u_to_string;
	group_u_Function_70* u_type;
};

struct group_u_BuiltinTag_30 {
	group_u_Function_70* u_type;
	group_u_Function_70* u_to_string;
	group_u_Object_32* u_internal_name;
};

struct group_u_Field_84 {
	group_u_Function_70* u_type;
	group_u_Function_70* u_to_string;
	group_u_Object_32* u_name;
	group_u_Object_32* u_value;
};

struct group_u_Error_132 {
	group_u_Function_70* u_type;
	group_u_Function_70* u_to_string;
	group_u_Object_32* u_message;
};

struct group_u_Group_26 {
	group_u_Function_70* u_type;
	group_u_Function_70* u_to_string;
	group_u_Object_32* u_fields;
};

struct type_u_anonymous_object_99 {
	group_u_Function_70* u_type;
	group_u_Function_70* u_to_string;
};

struct group_u_This_7 {
	group_u_Function_70* u_type;
	group_u_Function_70* u_to_string;
};

struct type_u_anonymous_object_102 {
	group_u_Function_70* u_type;
	group_u_Function_70* u_to_string;
};

struct group_u_Object_32 {
	group_u_Function_70* u_type;
	group_u_Function_70* u_to_string;
};

struct type_u_terminal_152 {
	group_u_Function_70* u_type;
	group_u_Function_70* u_to_string;
	group_u_Function_70* u_print;
	group_u_Function_70* u_type;
	group_u_Function_70* u_to_string;
	group_u_Function_70* u_input;
};

struct group_u_Either_94 {
	group_u_Function_70* u_type;
	group_u_Function_70* u_to_string;
	group_u_Object_32* u_variants;
};

struct type_u_anonymous_object_10 {
	group_u_Function_70* u_type;
	group_u_Function_70* u_to_string;
};

struct group_u_Parameter_60 {
	group_u_Function_70* u_type;
	group_u_Function_70* u_to_string;
	group_u_Object_32* u_name;
	group_u_Object_32* u_type;
};

struct group_u_List_91 {
	group_u_Function_70* u_type;
	group_u_Function_70* u_to_string;
	void* elements;
	int size;
	int capacity;
};

struct type_u_cabin_only_33 {
	group_u_Function_70* u_type;
	group_u_Function_70* u_to_string;
};

void call_anonymous_function_116(group_u_Number_107* u_this, group_u_Number_107* u_other, group_u_Number_107* return_address) {
	*return_address = (group_u_Number_107) { .internal_value = u_this->internal_value + u_other->internal_value };
}

void call_anonymous_function_158(group_u_Anything_22* u_object) {
	group_u_Text_19* return_address;
	(((void (*)(group_u_Anything_22*, group_u_Text_19*))(u_object->u_to_string->call))(u_object, return_address));
	printf("%s\n", return_address->internal_value);
}

void call_anonymous_function_167(group_u_Number_107* u_this, group_u_Number_107* u_other, group_u_Number_107* return_address) {
	*return_address = (group_u_Number_107) { .internal_value = u_this->internal_value + u_other->internal_value };
}

void call_anonymous_function_125(group_u_Number_107* u_this, group_u_Number_107* u_other, group_u_Number_107* return_address) {

}

void call_anonymous_function_146(group_u_Anything_22* u_object) {
	group_u_Text_19* return_address;
	(((void (*)(group_u_Anything_22*, group_u_Text_19*))(u_object->u_to_string->call))(u_object, return_address));
	printf("%s\n", return_address->internal_value);
}

void call_anonymous_function_53(group_u_Anything_22* u_this, group_u_Text_19* return_address) {
	// Get the type metadata of the value
	group_u_Group_26 type;
	(((void (*)(group_u_Anything_22*, group_u_Group_26*))(u_this->u_type->call))(u_this, &type));
	
	// Build the string
	DynamicString result = (DynamicString) { .value = type.name, .capacity = 16 };
	push_to_dynamic_string(&result, " {");
	
	// Add fields
	for (int fieldNumber = 0; fieldNumber < type.u_fields->elements.size; fieldNumber++) {										
		push_to_dynamic_string(&result, (char*) type.u_fields->elements.data[fieldNumber]);
	}
	
	// Return the built string
	push_to_dynamic_string(&result, " }");
	*return_address = (group_u_Text_19) { .internal_value = result.value };
}

void call_anonymous_function_46(group_u_Anything_22* u_this, group_u_Group_26* return_address) {

}

void call_anonymous_function_151(group_u_Text_19* return_address) {
	char* buffer = malloc(sizeof(char) * 256);
	fgets(buffer, 256, stdin);
	*return_address = (group_u_Text_19) { .internal_value = buffer };
}

int main(int argc, char** argv) {
	group_u_List_91* u_anonymous_list_122 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_List_91* u_anonymous_list_150 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_List_91* u_anonymous_list_81 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Group_26* u_Function_70 = &(group_u_Group_26) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_fields = u_anonymous_list_81,
	};
	
	group_u_List_91* u_anonymous_list_130 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Group_26* u_Number_107 = &(group_u_Group_26) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_fields = u_anonymous_list_130,
	};
	
	group_u_List_91* u_anonymous_list_123 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_List_91* u_anonymous_list_124 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Object_32* u_anonymous_object_10 = &(type_u_anonymous_object_10) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Function_70* anonymous_function_125 = &(group_u_Function_70) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_return_type = u_Number_107,
		.u_parameters = u_anonymous_list_123,
		.u_tags = u_anonymous_list_124,
		.u_compile_time_parameters = u_anonymous_list_122,
		.u_this_object = u_anonymous_object_10,
		.call = &call_anonymous_function_125
	};
	
	group_u_Text_19* anonymous_string_literal_128 = &(group_u_Text_19) { .internal_value = "minus" };
	
	group_u_Field_84* u_Number_minus_129 = &(group_u_Field_84) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_value = anonymous_function_125,
		.u_name = anonymous_string_literal_128,
	};
	
	group_u_List_91* u_anonymous_list_58 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Group_26* u_Anything_22 = &(group_u_Group_26) {
		.u_fields = u_anonymous_list_58,
	};
	
	group_u_Text_19* anonymous_string_literal_153 = &(group_u_Text_19) { .internal_value = "object" };
	
	group_u_Parameter_60* u_anonymous_function_object_154 = &(group_u_Parameter_60) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_type = u_Anything_22,
		.u_name = anonymous_string_literal_153,
	};
	
	group_u_List_91* u_anonymous_list_156 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_List_91* u_anonymous_list_115 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_List_91* u_anonymous_list_113 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_List_91* u_anonymous_list_114 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Function_70* anonymous_function_116 = &(group_u_Function_70) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_return_type = u_Number_107,
		.u_tags = u_anonymous_list_115,
		.u_this_object = u_anonymous_object_10,
		.u_compile_time_parameters = u_anonymous_list_113,
		.u_parameters = u_anonymous_list_114,
		.call = &call_anonymous_function_116
	};
	
	group_u_List_91* u_anonymous_list_50 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_List_91* u_anonymous_list_51 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_List_91* u_anonymous_list_20 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Group_26* u_Text_19 = &(group_u_Group_26) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_fields = u_anonymous_list_20,
	};
	
	group_u_List_91* u_anonymous_list_52 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Function_70* anonymous_function_53 = &(group_u_Function_70) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_this_object = u_anonymous_object_10,
		.u_compile_time_parameters = u_anonymous_list_50,
		.u_parameters = u_anonymous_list_51,
		.u_return_type = u_Text_19,
		.u_tags = u_anonymous_list_52,
		.call = &call_anonymous_function_53
	};
	
	group_u_List_91* u_anonymous_list_44 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_List_91* u_anonymous_list_43 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_List_91* u_anonymous_list_45 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_List_91* u_anonymous_list_25 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Group_26* u_Group_26 = &(group_u_Group_26) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_fields = u_anonymous_list_25,
	};
	
	group_u_Function_70* anonymous_function_46 = &(group_u_Function_70) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_this_object = u_anonymous_object_10,
		.u_parameters = u_anonymous_list_44,
		.u_compile_time_parameters = u_anonymous_list_43,
		.u_tags = u_anonymous_list_45,
		.u_return_type = u_Group_26,
		.call = &call_anonymous_function_46
	};
	
	group_u_Number_107* u_anonymous_number_159 = &(group_u_Number_107) { .internal_value = 3 };
	
	group_u_Text_19* anonymous_string_literal_9 = &(group_u_Text_19) { .internal_value = "nothing" };
	
	group_u_List_91* u_anonymous_list_37 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Text_19* anonymous_string_literal_111 = &(group_u_Text_19) { .internal_value = "other" };
	
	group_u_Parameter_60* u_anonymous_function_other_112 = &(group_u_Parameter_60) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_name = anonymous_string_literal_111,
		.u_type = u_Number_107,
	};
	
	group_u_Text_19* anonymous_string_literal_0 = &(group_u_Text_19) { .internal_value = "Anything.type" };
	
	group_u_BuiltinTag_30* anonymous_object_40 = &(group_u_BuiltinTag_30) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_internal_name = anonymous_string_literal_0,
	};
	
	group_u_Text_19* anonymous_string_literal_87 = &(group_u_Text_19) { .internal_value = "value" };
	
	group_u_List_91* u_anonymous_list_66 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Text_19* anonymous_string_literal_4 = &(group_u_Text_19) { .internal_value = "terminal.print" };
	
	group_u_BuiltinTag_30* anonymous_object_140 = &(group_u_BuiltinTag_30) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_internal_name = anonymous_string_literal_4,
	};
	
	group_u_List_91* u_anonymous_list_155 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Number_107* u_anonymous_number_168 = &(group_u_Number_107) { .internal_value = 4 };
	
	group_u_Text_19* anonymous_string_literal_162 = &(group_u_Text_19) { .internal_value = "other" };
	
	group_u_Parameter_60* u_anonymous_function_other_163 = &(group_u_Parameter_60) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_name = anonymous_string_literal_162,
		.u_type = u_Number_107,
	};
	
	group_u_Text_19* anonymous_string_literal_23 = &(group_u_Text_19) { .internal_value = "fields" };
	
	group_u_Text_19* anonymous_string_literal_5 = &(group_u_Text_19) { .internal_value = "terminal.input" };
	
	group_u_Text_19* anonymous_string_literal_126 = &(group_u_Text_19) { .internal_value = "plus" };
	
	group_u_Text_19* anonymous_string_literal_75 = &(group_u_Text_19) { .internal_value = "compile_time_parameters" };
	
	group_u_Text_19* anonymous_string_literal_109 = &(group_u_Text_19) { .internal_value = "this" };
	
	group_u_Parameter_60* u_anonymous_function_this_110 = &(group_u_Parameter_60) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_name = anonymous_string_literal_109,
		.u_type = u_Number_107,
	};
	
	group_u_List_91* u_anonymous_list_68 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Group_26* u_OneOf_67 = &(group_u_Group_26) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_fields = u_anonymous_list_68,
	};
	
	group_u_Text_19* anonymous_string_literal_27 = &(group_u_Text_19) { .internal_value = "internal_name" };
	
	group_u_Field_84* u_BuiltinTag_internal_name_28 = &(group_u_Field_84) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_name = anonymous_string_literal_27,
		.u_value = u_anonymous_object_10,
	};
	
	group_u_Text_19* anonymous_string_literal_95 = &(group_u_Text_19) { .internal_value = "variants" };
	
	group_u_Field_84* u_Either_variants_96 = &(group_u_Field_84) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_value = u_anonymous_object_10,
		.u_name = anonymous_string_literal_95,
	};
	
	group_u_Text_19* anonymous_string_literal_77 = &(group_u_Text_19) { .internal_value = "tags" };
	
	group_u_Object_32* u_system_side_effects_82 = &(type_u_system_side_effects_82) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_to_string = anonymous_function_53,
		.u_type = anonymous_function_46,
	};
	
	group_u_Text_19* anonymous_string_literal_118 = &(group_u_Text_19) { .internal_value = "this" };
	
	group_u_List_91* u_anonymous_list_157 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_List_91* u_anonymous_list_144 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_List_91* u_anonymous_list_145 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_List_91* u_anonymous_list_143 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Function_70* anonymous_function_146 = &(group_u_Function_70) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_parameters = u_anonymous_list_144,
		.u_this_object = u_anonymous_object_10,
		.u_return_type = u_anonymous_object_10,
		.u_tags = u_anonymous_list_145,
		.u_compile_time_parameters = u_anonymous_list_143,
		.call = &call_anonymous_function_146
	};
	
	group_u_List_91* u_anonymous_list_149 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_List_91* u_anonymous_list_148 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Function_70* anonymous_function_151 = &(group_u_Function_70) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_parameters = u_anonymous_list_149,
		.u_tags = u_anonymous_list_150,
		.u_this_object = u_anonymous_object_10,
		.u_return_type = u_Text_19,
		.u_compile_time_parameters = u_anonymous_list_148,
		.call = &call_anonymous_function_151
	};
	
	group_u_Object_32* u_terminal_152 = &(type_u_terminal_152) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_print = anonymous_function_146,
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_input = anonymous_function_151,
	};
	
	group_u_Function_70* anonymous_function_158 = &(group_u_Function_70) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_return_type = u_anonymous_object_10,
		.u_compile_time_parameters = u_anonymous_list_155,
		.u_tags = u_anonymous_list_157,
		.u_this_object = u_terminal_152,
		.u_parameters = u_anonymous_list_156,
		.call = &call_anonymous_function_158
	};
	
	group_u_List_91* u_anonymous_list_29 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Group_26* u_BuiltinTag_30 = &(group_u_Group_26) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_fields = u_anonymous_list_29,
	};
	
	group_u_Text_19* anonymous_string_literal_2 = &(group_u_Text_19) { .internal_value = "Number.plus" };
	
	group_u_Text_19* anonymous_string_literal_3 = &(group_u_Text_19) { .internal_value = "Number.minus" };
	
	group_u_Text_19* anonymous_string_literal_71 = &(group_u_Text_19) { .internal_value = "parameters" };
	
	group_u_Field_84* u_Function_parameters_72 = &(group_u_Field_84) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_value = u_anonymous_object_10,
		.u_name = anonymous_string_literal_71,
	};
	
	group_u_Field_84* u_Field_value_88 = &(group_u_Field_84) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_name = anonymous_string_literal_87,
		.u_value = u_anonymous_object_10,
	};
	
	group_u_Object_32* u_anonymous_object_99 = &(type_u_anonymous_object_99) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Text_19* anonymous_string_literal_98 = &(group_u_Text_19) { .internal_value = "true" };
	
	group_u_Field_84* u_true_100 = &(group_u_Field_84) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_value = u_anonymous_object_99,
		.u_name = anonymous_string_literal_98,
	};
	
	group_u_List_91* u_anonymous_list_38 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Field_84* u_Function_tags_78 = &(group_u_Field_84) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_value = u_anonymous_object_10,
		.u_name = anonymous_string_literal_77,
	};
	
	group_u_List_91* u_anonymous_list_89 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Group_26* u_Field_84 = &(group_u_Group_26) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_fields = u_anonymous_list_89,
	};
	
	group_u_List_91* u_anonymous_list_164 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_List_91* u_anonymous_list_165 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_List_91* u_anonymous_list_166 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Function_70* anonymous_function_167 = &(group_u_Function_70) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_parameters = u_anonymous_list_165,
		.u_return_type = u_Number_107,
		.u_tags = u_anonymous_list_166,
		.u_compile_time_parameters = u_anonymous_list_164,
		.u_this_object = u_anonymous_number_159,
		.call = &call_anonymous_function_167
	};
	
	group_u_Text_19* anonymous_string_literal_63 = &(group_u_Text_19) { .internal_value = "type" };
	
	group_u_Field_84* u_Parameter_type_64 = &(group_u_Field_84) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_value = u_anonymous_object_10,
		.u_name = anonymous_string_literal_63,
	};
	
	group_u_List_91* u_anonymous_list_97 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Text_19* anonymous_string_literal_133 = &(group_u_Text_19) { .internal_value = "message" };
	
	group_u_Text_19* anonymous_string_literal_48 = &(group_u_Text_19) { .internal_value = "this" };
	
	group_u_Text_19* anonymous_string_literal_61 = &(group_u_Text_19) { .internal_value = "name" };
	
	group_u_Field_84* u_Parameter_name_62 = &(group_u_Field_84) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_name = anonymous_string_literal_61,
		.u_value = u_anonymous_object_10,
	};
	
	group_u_Text_19* anonymous_string_literal_56 = &(group_u_Text_19) { .internal_value = "to_string" };
	
	group_u_List_91* u_anonymous_list_65 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_List_91* u_anonymous_list_12 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Text_19* anonymous_string_literal_15 = &(group_u_Text_19) { .internal_value = "Data" };
	
	group_u_List_91* u_anonymous_list_135 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_BuiltinTag_30* anonymous_object_147 = &(group_u_BuiltinTag_30) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_to_string = anonymous_function_53,
		.u_type = anonymous_function_46,
		.u_internal_name = anonymous_string_literal_5,
	};
	
	group_u_Text_19* anonymous_string_literal_160 = &(group_u_Text_19) { .internal_value = "this" };
	
	group_u_Text_19* anonymous_string_literal_34 = &(group_u_Text_19) { .internal_value = "name" };
	
	group_u_List_91* u_anonymous_list_6 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Parameter_60* u_anonymous_function_this_119 = &(group_u_Parameter_60) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_type = u_Number_107,
		.u_name = anonymous_string_literal_118,
	};
	
	group_u_Group_26* u_Error_132 = &(group_u_Group_26) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_fields = u_anonymous_list_135,
	};
	
	group_u_List_91* u_anonymous_list_18 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Text_19* anonymous_string_literal_85 = &(group_u_Text_19) { .internal_value = "name" };
	
	group_u_Field_84* u_Field_name_86 = &(group_u_Field_84) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_name = anonymous_string_literal_85,
		.u_value = u_anonymous_object_10,
	};
	
	group_u_List_91* u_anonymous_list_93 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Text_19* anonymous_string_literal_73 = &(group_u_Text_19) { .internal_value = "return_type" };
	
	group_u_Field_84* u_Function_return_type_74 = &(group_u_Field_84) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_value = u_anonymous_object_10,
		.u_name = anonymous_string_literal_73,
	};
	
	group_u_Text_19* anonymous_string_literal_54 = &(group_u_Text_19) { .internal_value = "type" };
	
	group_u_Field_84* u_Anything_type_55 = &(group_u_Field_84) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_value = anonymous_function_46,
		.u_name = anonymous_string_literal_54,
	};
	
	group_u_Text_19* anonymous_string_literal_101 = &(group_u_Text_19) { .internal_value = "false" };
	
	group_u_List_91* u_anonymous_list_104 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_List_91* u_anonymous_list_131 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_BuiltinTag_30* anonymous_object_117 = &(group_u_BuiltinTag_30) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_type = anonymous_function_46,
		.u_internal_name = anonymous_string_literal_3,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_List_91* u_anonymous_list_90 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_List_91* u_anonymous_list_8 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Group_26* u_This_7 = &(group_u_Group_26) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_fields = u_anonymous_list_8,
	};
	
	group_u_Parameter_60* u_builtin_name_35 = &(group_u_Parameter_60) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_type = u_Text_19,
		.u_name = anonymous_string_literal_34,
	};
	
	group_u_Text_19* anonymous_string_literal_120 = &(group_u_Text_19) { .internal_value = "other" };
	
	group_u_List_91* u_anonymous_list_138 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_List_91* u_anonymous_list_92 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Text_19* anonymous_string_literal_137 = &(group_u_Text_19) { .internal_value = "Data" };
	
	group_u_Field_84* u_nothing_11 = &(group_u_Field_84) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_name = anonymous_string_literal_9,
		.u_value = u_anonymous_object_10,
	};
	
	group_u_List_91* u_anonymous_list_83 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Parameter_60* u_anonymous_function_this_49 = &(group_u_Parameter_60) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_name = anonymous_string_literal_48,
		.u_type = u_Anything_22,
	};
	
	group_u_Field_84* u_Group_fields_24 = &(group_u_Field_84) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_value = u_anonymous_object_10,
		.u_name = anonymous_string_literal_23,
	};
	
	group_u_Text_19* anonymous_string_literal_41 = &(group_u_Text_19) { .internal_value = "this" };
	
	group_u_List_91* u_anonymous_list_59 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Object_32* u_anonymous_object_102 = &(type_u_anonymous_object_102) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Field_84* u_false_103 = &(group_u_Field_84) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_name = anonymous_string_literal_101,
		.u_value = u_anonymous_object_102,
	};
	
	group_u_Parameter_60* u_anonymous_function_this_161 = &(group_u_Parameter_60) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_name = anonymous_string_literal_160,
		.u_type = u_Number_107,
	};
	
	group_u_List_91* u_anonymous_list_36 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Field_84* u_Anything_to_string_57 = &(group_u_Field_84) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_name = anonymous_string_literal_56,
		.u_value = anonymous_function_53,
	};
	
	group_u_Field_84* u_Function_compile_time_parameters_76 = &(group_u_Field_84) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_value = u_anonymous_object_10,
		.u_name = anonymous_string_literal_75,
	};
	
	group_u_Text_19* anonymous_string_literal_1 = &(group_u_Text_19) { .internal_value = "Anything.to_string" };
	
	group_u_BuiltinTag_30* anonymous_object_47 = &(group_u_BuiltinTag_30) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_internal_name = anonymous_string_literal_1,
	};
	
	group_u_Parameter_60* u_anonymous_function_this_42 = &(group_u_Parameter_60) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_type = u_Anything_22,
		.u_name = anonymous_string_literal_41,
	};
	
	group_u_Parameter_60* u_anonymous_function_other_121 = &(group_u_Parameter_60) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_type = u_Number_107,
		.u_name = anonymous_string_literal_120,
	};
	
	group_u_List_91* u_anonymous_list_31 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Group_26* u_Object_32 = &(group_u_Group_26) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_fields = u_anonymous_list_31,
	};
	
	group_u_List_91* u_anonymous_list_14 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_List_91* u_anonymous_list_69 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_List_91* u_anonymous_list_16 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Group_26* u_Either_94 = &(group_u_Group_26) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_fields = u_anonymous_list_97,
	};
	
	group_u_Text_19* anonymous_string_literal_141 = &(group_u_Text_19) { .internal_value = "object" };
	
	group_u_Parameter_60* u_anonymous_function_object_142 = &(group_u_Parameter_60) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_name = anonymous_string_literal_141,
		.u_type = u_Anything_22,
	};
	
	group_u_Text_19* anonymous_string_literal_79 = &(group_u_Text_19) { .internal_value = "this_object" };
	
	group_u_Field_84* u_Function_this_object_80 = &(group_u_Field_84) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_value = u_anonymous_object_10,
		.u_name = anonymous_string_literal_79,
	};
	
	group_u_Field_84* u_Number_plus_127 = &(group_u_Field_84) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_value = anonymous_function_116,
		.u_name = anonymous_string_literal_126,
	};
	
	group_u_List_91* u_anonymous_list_106 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_BuiltinTag_30* anonymous_object_108 = &(group_u_BuiltinTag_30) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_internal_name = anonymous_string_literal_2,
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_List_91* u_anonymous_list_21 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Group_26* u_Parameter_60 = &(group_u_Group_26) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_fields = u_anonymous_list_65,
	};
	
	group_u_Field_84* u_Error_message_134 = &(group_u_Field_84) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_name = anonymous_string_literal_133,
		.u_value = u_anonymous_object_10,
	};
	
	group_u_List_91* u_anonymous_list_136 = &(group_u_List_91) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
	group_u_Group_26* u_List_91 = &(group_u_Group_26) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
		.u_fields = u_anonymous_list_92,
	};
	
	group_u_Object_32* u_cabin_only_33 = &(type_u_cabin_only_33) {
		.u_type = anonymous_function_46,
		.u_to_string = anonymous_function_53,
	};
	
				({
						
					group_u_Number_107* arg0 = ({
		group_u_Number_107* return_address;	
		group_u_Number_107* arg0 = u_anonymous_number_168;
		(((void (*)(group_u_Number_107*, group_u_Number_107*, group_u_Number_107*))(anonymous_function_167->call))(u_anonymous_number_159, arg0, return_address));
		return_address;
	})	
	;
					(((void (*)(group_u_Anything_22*))(anonymous_function_158->call))(arg0));
					
				})	
				;

	return 0;
}