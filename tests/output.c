#include <stdio.h>
#include <stdlib.h>

typedef struct group_u_BuiltinTag_72 group_u_BuiltinTag_72;
typedef struct group_u_Number_102 group_u_Number_102;
typedef struct group_u_Anything_20 group_u_Anything_20;
typedef struct type_u_anonymous_object_65 type_u_anonymous_object_65;
typedef enum either_u_Boolean_68 u_Boolean_68;typedef struct group_u_Function_46 group_u_Function_46;
typedef struct group_u_Error_106 group_u_Error_106;
typedef struct type_u_terminal_123 type_u_terminal_123;
typedef struct group_u_Parameter_32 group_u_Parameter_32;
typedef struct type_u_cabin_only_47 type_u_cabin_only_47;
typedef enum either_u_Nothing_12 u_Nothing_12;typedef struct group_u_OneOf_34 group_u_OneOf_34;
typedef struct type_u_anonymous_object_62 type_u_anonymous_object_62;
typedef struct group_u_List_56 group_u_List_56;
typedef struct group_u_Text_18 group_u_Text_18;
typedef struct group_u_Either_60 group_u_Either_60;
typedef struct group_u_Field_54 group_u_Field_54;
typedef struct type_u_anonymous_object_9 type_u_anonymous_object_9;
typedef struct group_u_This_7 group_u_This_7;
typedef struct group_u_Group_26 group_u_Group_26;
typedef struct type_u_system_side_effects_48 type_u_system_side_effects_48;
typedef struct group_u_Object_22 group_u_Object_22;

struct group_u_BuiltinTag_72 {
	void* u_internal_name;
};

struct group_u_Number_102 {
	void* u_plus;
	void* u_minus;
};

struct group_u_Anything_20 {
	char empty;
};

struct type_u_anonymous_object_65 {
	char empty;
};

enum u_Boolean {

	u_true,
	u_false,
};

struct group_u_Function_46 {
	void* u_parameters;
	void* u_return_type;
	void* u_compile_time_parameters;
	void* u_tags;
	void* u_this_object;
	void* call;
};

struct group_u_Error_106 {
	void* u_message;
};

struct type_u_terminal_123 {
	group_u_Function_46* u_print;
	group_u_Function_46* u_input;
};

struct group_u_Parameter_32 {
	void* u_name;
	void* u_type;
};

struct type_u_cabin_only_47 {
	char empty;
};

enum u_Nothing {

	u_nothing,
};

struct group_u_OneOf_34 {
	char empty;
};

struct type_u_anonymous_object_62 {
	char empty;
};

struct group_u_List_56 {
	char empty;
};

struct group_u_Text_18 {
	char* internal_value;
};

struct group_u_Either_60 {
	void* u_variants;
};

struct group_u_Field_54 {
	void* u_name;
	void* u_value;
};

struct type_u_anonymous_object_9 {
	char empty;
};

struct group_u_This_7 {
	char empty;
};

struct group_u_Group_26 {
	void* u_fields;
};

struct type_u_system_side_effects_48 {
	char empty;
};

struct group_u_Object_22 {
	char empty;
};

void call_anonymous_function_129(group_u_Anything_20* u_object) {
	printf("%s\n", ((group_u_Text_18*) u_object)->internal_value);
}

void call_anonymous_function_96(group_u_Anything_20* u_this, group_u_Anything_20* u_other, group_u_Anything_20* return_address) {

}

void call_anonymous_function_117(group_u_Anything_20* u_object) {
	printf("%s\n", ((group_u_Text_18*) u_object)->internal_value);
}

void call_anonymous_function_87(group_u_Anything_20* u_this, group_u_Anything_20* u_other, group_u_Anything_20* return_address) {

}

void call_anonymous_function_122(group_u_Text_18* return_address) {
	char* buffer = malloc(sizeof(char) * 256);
	fgets(buffer, 256, stdin);
	*return_address = (group_u_Text_18) { .internal_value = buffer };
}

int main(int argc, char** argv) {
	group_u_Text_18* anonymous_string_literal_112 = &(group_u_Text_18) { .internal_value = "object" };
	
	group_u_List_56* u_anonymous_list_19 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_Group_26* u_Anything_20 = &(group_u_Group_26) {
		.u_fields = u_anonymous_list_19,
	};
	
	group_u_Parameter_32* u_anonymous_function_object_113 = &(group_u_Parameter_32) {
		.u_name = anonymous_string_literal_112,
		.u_type = u_Anything_20,
	};
	
	group_u_List_56* u_anonymous_list_17 = &(group_u_List_56) {
		.empty = '0'
	};
	
	type_u_anonymous_object_9* u_anonymous_object_9 = &(type_u_anonymous_object_9) {
		.empty = '0'
	};
	
	group_u_Text_18* anonymous_string_literal_29 = &(group_u_Text_18) { .internal_value = "type" };
	
	group_u_Field_54* u_Parameter_type_30 = &(group_u_Field_54) {
		.u_value = u_anonymous_object_9,
		.u_name = anonymous_string_literal_29,
	};
	
	group_u_Text_18* anonymous_string_literal_49 = &(group_u_Text_18) { .internal_value = "name" };
	
	group_u_Field_54* u_Field_name_50 = &(group_u_Field_54) {
		.u_value = u_anonymous_object_9,
		.u_name = anonymous_string_literal_49,
	};
	
	group_u_Text_18* anonymous_string_literal_61 = &(group_u_Text_18) { .internal_value = "true" };
	
	group_u_List_56* u_anonymous_list_71 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_Group_26* u_BuiltinTag_72 = &(group_u_Group_26) {
		.u_fields = u_anonymous_list_71,
	};
	
	group_u_List_56* u_anonymous_list_101 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_Group_26* u_Number_102 = &(group_u_Group_26) {
		.u_fields = u_anonymous_list_101,
	};
	
	group_u_List_56* u_anonymous_list_6 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_List_56* u_anonymous_list_128 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_List_56* u_anonymous_list_119 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_Text_18* anonymous_string_literal_99 = &(group_u_Text_18) { .internal_value = "minus" };
	
	group_u_List_56* u_anonymous_list_93 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_List_56* u_anonymous_list_94 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_List_56* u_anonymous_list_95 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_Function_46* anonymous_function_96 = &(group_u_Function_46) {
		.u_this_object = u_anonymous_object_9,
		.u_return_type = u_Anything_20,
		.u_compile_time_parameters = u_anonymous_list_93,
		.u_parameters = u_anonymous_list_94,
		.u_tags = u_anonymous_list_95,
		.call = &call_anonymous_function_96
	};
	
	group_u_Field_54* u_Number_minus_100 = &(group_u_Field_54) {
		.u_name = anonymous_string_literal_99,
		.u_value = anonymous_function_96,
	};
	
	group_u_List_56* u_anonymous_list_127 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_Text_18* anonymous_string_literal_14 = &(group_u_Text_18) { .internal_value = "Data" };
	
	type_u_anonymous_object_65* u_anonymous_object_65 = &(type_u_anonymous_object_65) {
		.empty = '0'
	};
	
	group_u_Text_18* anonymous_string_literal_108 = &(group_u_Text_18) { .internal_value = "Data" };
	
	group_u_List_56* u_anonymous_list_116 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_List_56* u_anonymous_list_114 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_List_56* u_anonymous_list_115 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_Function_46* anonymous_function_117 = &(group_u_Function_46) {
		.u_return_type = u_anonymous_object_9,
		.u_this_object = u_anonymous_object_9,
		.u_tags = u_anonymous_list_116,
		.u_compile_time_parameters = u_anonymous_list_114,
		.u_parameters = u_anonymous_list_115,
		.call = &call_anonymous_function_117
	};
	
	group_u_List_56* u_anonymous_list_120 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_Group_26* u_Text_18 = &(group_u_Group_26) {
		.u_fields = u_anonymous_list_17,
	};
	
	group_u_List_56* u_anonymous_list_121 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_Function_46* anonymous_function_122 = &(group_u_Function_46) {
		.u_this_object = u_anonymous_object_9,
		.u_parameters = u_anonymous_list_120,
		.u_compile_time_parameters = u_anonymous_list_119,
		.u_return_type = u_Text_18,
		.u_tags = u_anonymous_list_121,
		.call = &call_anonymous_function_122
	};
	
	type_u_terminal_123* u_terminal_123 = &(type_u_terminal_123) {
		.u_print = anonymous_function_117,
		.u_input = anonymous_function_122,
	};
	
	group_u_List_56* u_anonymous_list_126 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_Function_46* anonymous_function_129 = &(group_u_Function_46) {
		.u_this_object = u_terminal_123,
		.u_parameters = u_anonymous_list_127,
		.u_return_type = u_anonymous_object_9,
		.u_tags = u_anonymous_list_128,
		.u_compile_time_parameters = u_anonymous_list_126,
		.call = &call_anonymous_function_129
	};
	
	group_u_List_56* u_anonymous_list_107 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_Text_18* anonymous_string_literal_2 = &(group_u_Text_18) { .internal_value = "terminal.print" };
	
	group_u_BuiltinTag_72* anonymous_object_111 = &(group_u_BuiltinTag_72) {
		.u_internal_name = anonymous_string_literal_2,
	};
	
	group_u_List_56* u_anonymous_list_45 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_Group_26* u_Function_46 = &(group_u_Group_26) {
		.u_fields = u_anonymous_list_45,
	};
	
	group_u_List_56* u_anonymous_list_105 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_Group_26* u_Error_106 = &(group_u_Group_26) {
		.u_fields = u_anonymous_list_105,
	};
	
	group_u_List_56* u_anonymous_list_55 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_Text_18* anonymous_string_literal_64 = &(group_u_Text_18) { .internal_value = "false" };
	
	group_u_Field_54* u_false_66 = &(group_u_Field_54) {
		.u_value = u_anonymous_object_65,
		.u_name = anonymous_string_literal_64,
	};
	
	group_u_Text_18* anonymous_string_literal_80 = &(group_u_Text_18) { .internal_value = "this" };
	
	group_u_Parameter_32* u_anonymous_function_this_81 = &(group_u_Parameter_32) {
		.u_name = anonymous_string_literal_80,
		.u_type = u_Anything_20,
	};
	
	group_u_List_56* u_anonymous_list_15 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_Text_18* anonymous_string_literal_89 = &(group_u_Text_18) { .internal_value = "this" };
	
	group_u_Parameter_32* u_anonymous_function_this_90 = &(group_u_Parameter_32) {
		.u_name = anonymous_string_literal_89,
		.u_type = u_Anything_20,
	};
	
	group_u_Text_18* anonymous_string_literal_39 = &(group_u_Text_18) { .internal_value = "compile_time_parameters" };
	
	group_u_List_56* u_anonymous_list_109 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_List_56* u_anonymous_list_25 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_Text_18* anonymous_string_literal_73 = &(group_u_Text_18) { .internal_value = "name" };
	
	group_u_Parameter_32* u_builtin_name_74 = &(group_u_Parameter_32) {
		.u_type = u_Text_18,
		.u_name = anonymous_string_literal_73,
	};
	
	group_u_Text_18* anonymous_string_literal_91 = &(group_u_Text_18) { .internal_value = "other" };
	
	group_u_Parameter_32* u_anonymous_function_other_92 = &(group_u_Parameter_32) {
		.u_name = anonymous_string_literal_91,
		.u_type = u_Anything_20,
	};
	
	group_u_List_56* u_anonymous_list_53 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_Text_18* anonymous_string_literal_37 = &(group_u_Text_18) { .internal_value = "return_type" };
	
	group_u_List_56* u_anonymous_list_31 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_Text_18* anonymous_string_literal_82 = &(group_u_Text_18) { .internal_value = "other" };
	
	group_u_List_56* u_anonymous_list_33 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_Group_26* u_Parameter_32 = &(group_u_Group_26) {
		.u_fields = u_anonymous_list_31,
	};
	
	group_u_Text_18* anonymous_string_literal_23 = &(group_u_Text_18) { .internal_value = "fields" };
	
	group_u_Text_18* anonymous_string_literal_57 = &(group_u_Text_18) { .internal_value = "variants" };
	
	group_u_Text_18* anonymous_string_literal_103 = &(group_u_Text_18) { .internal_value = "message" };
	
	group_u_Field_54* u_Error_message_104 = &(group_u_Field_54) {
		.u_name = anonymous_string_literal_103,
		.u_value = u_anonymous_object_9,
	};
	
	group_u_Text_18* anonymous_string_literal_1 = &(group_u_Text_18) { .internal_value = "Number.minus" };
	
	group_u_Text_18* anonymous_string_literal_0 = &(group_u_Text_18) { .internal_value = "Number.plus" };
	
	group_u_Text_18* anonymous_string_literal_5 = &(group_u_Text_18) { .internal_value = "Hello from compile-time!" };
	
	type_u_cabin_only_47* u_cabin_only_47 = &(type_u_cabin_only_47) {
		.empty = '0'
	};
	
	group_u_Text_18* anonymous_string_literal_69 = &(group_u_Text_18) { .internal_value = "internal_name" };
	
	group_u_Text_18* anonymous_string_literal_35 = &(group_u_Text_18) { .internal_value = "parameters" };
	
	group_u_Field_54* u_Function_parameters_36 = &(group_u_Field_54) {
		.u_name = anonymous_string_literal_35,
		.u_value = u_anonymous_object_9,
	};
	
	group_u_Text_18* anonymous_string_literal_3 = &(group_u_Text_18) { .internal_value = "terminal.input" };
	
	group_u_Text_18* anonymous_string_literal_41 = &(group_u_Text_18) { .internal_value = "tags" };
	
	group_u_List_56* u_anonymous_list_59 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_Group_26* u_OneOf_34 = &(group_u_Group_26) {
		.u_fields = u_anonymous_list_33,
	};
	
	group_u_Text_18* anonymous_string_literal_124 = &(group_u_Text_18) { .internal_value = "object" };
	
	group_u_Parameter_32* u_anonymous_function_object_125 = &(group_u_Parameter_32) {
		.u_name = anonymous_string_literal_124,
		.u_type = u_Anything_20,
	};
	
	group_u_Field_54* u_Function_compile_time_parameters_40 = &(group_u_Field_54) {
		.u_value = u_anonymous_object_9,
		.u_name = anonymous_string_literal_39,
	};
	
	group_u_Text_18* anonymous_string_literal_51 = &(group_u_Text_18) { .internal_value = "value" };
	
	group_u_Field_54* u_Field_value_52 = &(group_u_Field_54) {
		.u_value = u_anonymous_object_9,
		.u_name = anonymous_string_literal_51,
	};
	
	group_u_Text_18* anonymous_string_literal_97 = &(group_u_Text_18) { .internal_value = "plus" };
	
	group_u_Text_18* anonymous_string_literal_27 = &(group_u_Text_18) { .internal_value = "name" };
	
	group_u_Field_54* u_Parameter_name_28 = &(group_u_Field_54) {
		.u_name = anonymous_string_literal_27,
		.u_value = u_anonymous_object_9,
	};
	
	group_u_List_56* u_anonymous_list_77 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_Field_54* u_Function_return_type_38 = &(group_u_Field_54) {
		.u_value = u_anonymous_object_9,
		.u_name = anonymous_string_literal_37,
	};
	
	group_u_List_56* u_anonymous_list_75 = &(group_u_List_56) {
		.empty = '0'
	};
	
	type_u_anonymous_object_62* u_anonymous_object_62 = &(type_u_anonymous_object_62) {
		.empty = '0'
	};
	
	group_u_Field_54* u_true_63 = &(group_u_Field_54) {
		.u_value = u_anonymous_object_62,
		.u_name = anonymous_string_literal_61,
	};
	
	group_u_List_56* u_anonymous_list_85 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_List_56* u_anonymous_list_84 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_List_56* u_anonymous_list_86 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_Function_46* anonymous_function_87 = &(group_u_Function_46) {
		.u_parameters = u_anonymous_list_85,
		.u_compile_time_parameters = u_anonymous_list_84,
		.u_this_object = u_anonymous_object_9,
		.u_tags = u_anonymous_list_86,
		.u_return_type = u_Anything_20,
		.call = &call_anonymous_function_87
	};
	
	group_u_Field_54* u_BuiltinTag_internal_name_70 = &(group_u_Field_54) {
		.u_name = anonymous_string_literal_69,
		.u_value = u_anonymous_object_9,
	};
	
	group_u_List_56* u_anonymous_list_21 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_Text_18* anonymous_string_literal_8 = &(group_u_Text_18) { .internal_value = "nothing" };
	
	group_u_Field_54* u_nothing_10 = &(group_u_Field_54) {
		.u_value = u_anonymous_object_9,
		.u_name = anonymous_string_literal_8,
	};
	
	group_u_Text_18* anonymous_string_literal_130 = &(group_u_Text_18) { .internal_value = "Hello from compile-time!" };
	
	group_u_Group_26* u_List_56 = &(group_u_Group_26) {
		.u_fields = u_anonymous_list_55,
	};
	
	group_u_Field_54* u_Group_fields_24 = &(group_u_Field_54) {
		.u_value = u_anonymous_object_9,
		.u_name = anonymous_string_literal_23,
	};
	
	group_u_Parameter_32* u_anonymous_function_other_83 = &(group_u_Parameter_32) {
		.u_type = u_Anything_20,
		.u_name = anonymous_string_literal_82,
	};
	
	group_u_Text_18* anonymous_string_literal_43 = &(group_u_Text_18) { .internal_value = "this_object" };
	
	group_u_Field_54* u_Either_variants_58 = &(group_u_Field_54) {
		.u_name = anonymous_string_literal_57,
		.u_value = u_anonymous_object_9,
	};
	
	group_u_List_56* u_anonymous_list_11 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_Field_54* u_Function_this_object_44 = &(group_u_Field_54) {
		.u_name = anonymous_string_literal_43,
		.u_value = u_anonymous_object_9,
	};
	
	group_u_Field_54* u_Number_plus_98 = &(group_u_Field_54) {
		.u_value = anonymous_function_87,
		.u_name = anonymous_string_literal_97,
	};
	
	group_u_Group_26* u_Either_60 = &(group_u_Group_26) {
		.u_fields = u_anonymous_list_59,
	};
	
	group_u_List_56* u_anonymous_list_67 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_Group_26* u_Field_54 = &(group_u_Group_26) {
		.u_fields = u_anonymous_list_53,
	};
	
	group_u_Field_54* u_Function_tags_42 = &(group_u_Field_54) {
		.u_value = u_anonymous_object_9,
		.u_name = anonymous_string_literal_41,
	};
	
	group_u_Text_18* anonymous_string_literal_4 = &(group_u_Text_18) { .internal_value = "Hello world!" };
	
	group_u_Group_26* u_This_7 = &(group_u_Group_26) {
		.u_fields = u_anonymous_list_6,
	};
	
	group_u_Group_26* u_Group_26 = &(group_u_Group_26) {
		.u_fields = u_anonymous_list_25,
	};
	
	group_u_List_56* u_anonymous_list_13 = &(group_u_List_56) {
		.empty = '0'
	};
	
	type_u_system_side_effects_48* u_system_side_effects_48 = &(type_u_system_side_effects_48) {
		.empty = '0'
	};
	
	group_u_BuiltinTag_72* anonymous_object_79 = &(group_u_BuiltinTag_72) {
		.u_internal_name = anonymous_string_literal_0,
	};
	
	group_u_Group_26* u_Object_22 = &(group_u_Group_26) {
		.u_fields = u_anonymous_list_21,
	};
	
	group_u_List_56* u_anonymous_list_76 = &(group_u_List_56) {
		.empty = '0'
	};
	
	group_u_BuiltinTag_72* anonymous_object_118 = &(group_u_BuiltinTag_72) {
		.u_internal_name = anonymous_string_literal_3,
	};
	
	group_u_BuiltinTag_72* anonymous_object_88 = &(group_u_BuiltinTag_72) {
		.u_internal_name = anonymous_string_literal_1,
	};
	
	(((void (*)(group_u_Group_26*))(u_terminal_123->u_print->call))(anonymous_string_literal_4));
	void;

	return 0;
}