#include <stdio.h>
#include <stdlib.h>

typedef struct type_u_terminal_122 type_u_terminal_122;
typedef struct type_u_anonymous_object_61 type_u_anonymous_object_61;
typedef struct group_u_This_6 group_u_This_6;
typedef struct group_u_Error_105 group_u_Error_105;
typedef struct group_u_Anything_19 group_u_Anything_19;
typedef enum either_u_Nothing_11 u_Nothing_11;typedef struct group_u_Parameter_31 group_u_Parameter_31;
typedef struct group_u_Number_101 group_u_Number_101;
typedef struct group_u_Field_53 group_u_Field_53;
typedef struct group_u_Function_45 group_u_Function_45;
typedef struct group_u_Either_59 group_u_Either_59;
typedef struct type_u_cabin_only_46 type_u_cabin_only_46;
typedef struct type_u_anonymous_object_8 type_u_anonymous_object_8;
typedef struct group_u_List_55 group_u_List_55;
typedef struct group_u_OneOf_33 group_u_OneOf_33;
typedef struct group_u_BuiltinTag_71 group_u_BuiltinTag_71;
typedef struct group_u_Group_25 group_u_Group_25;
typedef enum either_u_Boolean_67 u_Boolean_67;typedef struct type_u_anonymous_object_64 type_u_anonymous_object_64;
typedef struct group_u_Text_17 group_u_Text_17;
typedef struct type_u_system_side_effects_47 type_u_system_side_effects_47;
typedef struct group_u_Object_21 group_u_Object_21;

struct type_u_terminal_122 {
	group_u_Function_45* u_print;
	group_u_Function_45* u_input;
};

struct type_u_anonymous_object_61 {
	char empty;
};

struct group_u_This_6 {
	char empty;
};

struct group_u_Error_105 {
	void* u_message;
};

struct group_u_Anything_19 {
	char empty;
};

enum u_Nothing {

	u_nothing,
};

struct group_u_Parameter_31 {
	void* u_name;
	void* u_type;
};

struct group_u_Number_101 {
	void* u_plus;
	void* u_minus;
};

struct group_u_Field_53 {
	void* u_name;
	void* u_value;
};

struct group_u_Function_45 {
	void* u_parameters;
	void* u_return_type;
	void* u_compile_time_parameters;
	void* u_tags;
	void* u_this_object;
	void* call;
};

struct group_u_Either_59 {
	void* u_variants;
};

struct type_u_cabin_only_46 {
	char empty;
};

struct type_u_anonymous_object_8 {
	char empty;
};

struct group_u_List_55 {
	char empty;
};

struct group_u_OneOf_33 {
	char empty;
};

struct group_u_BuiltinTag_71 {
	void* u_internal_name;
};

struct group_u_Group_25 {
	void* u_fields;
};

enum u_Boolean {

	u_true,
	u_false,
};

struct type_u_anonymous_object_64 {
	char empty;
};

struct group_u_Text_17 {
	char* internal_value;
};

struct type_u_system_side_effects_47 {
	char empty;
};

struct group_u_Object_21 {
	char empty;
};

void call_anonymous_function_95(group_u_Anything_19* u_this, group_u_Anything_19* u_other, group_u_Anything_19* return_address) {

}

void call_anonymous_function_121(group_u_Text_17* return_address) {
	char* buffer = malloc(sizeof(char) * 256);
	fgets(buffer, 256, stdin);
	*return_address = (group_u_Text_17) { .internal_value = buffer };
}

void call_anonymous_function_116(group_u_Anything_19* u_object) {
	printf("%s\n", ((group_u_Text_17*) u_object)->internal_value);
}

void call_anonymous_function_86(group_u_Anything_19* u_this, group_u_Anything_19* u_other, group_u_Anything_19* return_address) {

}

int main(int argc, char** argv) {
	group_u_Text_17* anonymous_string_literal_56 = &(group_u_Text_17) { .internal_value = "variants" };
	
	type_u_anonymous_object_8* u_anonymous_object_8 = &(type_u_anonymous_object_8) {
		.empty = '0'
	};
	
	group_u_List_55* u_anonymous_list_114 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_List_55* u_anonymous_list_113 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_List_55* u_anonymous_list_115 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_Function_45* anonymous_function_116 = &(group_u_Function_45) {
		.u_return_type = u_anonymous_object_8,
		.u_parameters = u_anonymous_list_114,
		.u_compile_time_parameters = u_anonymous_list_113,
		.u_tags = u_anonymous_list_115,
		.u_this_object = u_anonymous_object_8,
		.call = &call_anonymous_function_116
	};
	
	group_u_List_55* u_anonymous_list_120 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_List_55* u_anonymous_list_118 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_List_55* u_anonymous_list_119 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_List_55* u_anonymous_list_16 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_Group_25* u_Text_17 = &(group_u_Group_25) {
		.u_fields = u_anonymous_list_16,
	};
	
	group_u_Function_45* anonymous_function_121 = &(group_u_Function_45) {
		.u_tags = u_anonymous_list_120,
		.u_this_object = u_anonymous_object_8,
		.u_compile_time_parameters = u_anonymous_list_118,
		.u_parameters = u_anonymous_list_119,
		.u_return_type = u_Text_17,
		.call = &call_anonymous_function_121
	};
	
	type_u_terminal_122* u_terminal_122 = &(type_u_terminal_122) {
		.u_print = anonymous_function_116,
		.u_input = anonymous_function_121,
	};
	
	group_u_List_55* u_anonymous_list_20 = &(group_u_List_55) {
		.empty = '0'
	};
	
	type_u_anonymous_object_61* u_anonymous_object_61 = &(type_u_anonymous_object_61) {
		.empty = '0'
	};
	
	group_u_Text_17* anonymous_string_literal_60 = &(group_u_Text_17) { .internal_value = "true" };
	
	group_u_Field_53* u_true_62 = &(group_u_Field_53) {
		.u_name = anonymous_string_literal_60,
		.u_value = u_anonymous_object_61,
	};
	
	group_u_List_55* u_anonymous_list_5 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_Group_25* u_This_6 = &(group_u_Group_25) {
		.u_fields = u_anonymous_list_5,
	};
	
	group_u_Text_17* anonymous_string_literal_7 = &(group_u_Text_17) { .internal_value = "nothing" };
	
	group_u_Field_53* u_nothing_9 = &(group_u_Field_53) {
		.u_value = u_anonymous_object_8,
		.u_name = anonymous_string_literal_7,
	};
	
	group_u_Text_17* anonymous_string_literal_0 = &(group_u_Text_17) { .internal_value = "Number.plus" };
	
	group_u_Text_17* anonymous_string_literal_79 = &(group_u_Text_17) { .internal_value = "this" };
	
	group_u_List_55* u_anonymous_list_18 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_Group_25* u_Anything_19 = &(group_u_Group_25) {
		.u_fields = u_anonymous_list_18,
	};
	
	group_u_Parameter_31* u_anonymous_function_this_80 = &(group_u_Parameter_31) {
		.u_name = anonymous_string_literal_79,
		.u_type = u_Anything_19,
	};
	
	group_u_Text_17* anonymous_string_literal_63 = &(group_u_Text_17) { .internal_value = "false" };
	
	type_u_anonymous_object_64* u_anonymous_object_64 = &(type_u_anonymous_object_64) {
		.empty = '0'
	};
	
	group_u_Field_53* u_false_65 = &(group_u_Field_53) {
		.u_name = anonymous_string_literal_63,
		.u_value = u_anonymous_object_64,
	};
	
	group_u_Text_17* anonymous_string_literal_72 = &(group_u_Text_17) { .internal_value = "name" };
	
	group_u_Parameter_31* u_builtin_name_73 = &(group_u_Parameter_31) {
		.u_type = u_Text_17,
		.u_name = anonymous_string_literal_72,
	};
	
	group_u_Text_17* anonymous_string_literal_2 = &(group_u_Text_17) { .internal_value = "terminal.print" };
	
	group_u_BuiltinTag_71* anonymous_object_110 = &(group_u_BuiltinTag_71) {
		.u_internal_name = anonymous_string_literal_2,
	};
	
	group_u_Text_17* anonymous_string_literal_102 = &(group_u_Text_17) { .internal_value = "message" };
	
	group_u_Field_53* u_Error_message_103 = &(group_u_Field_53) {
		.u_name = anonymous_string_literal_102,
		.u_value = u_anonymous_object_8,
	};
	
	group_u_List_55* u_anonymous_list_108 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_List_55* u_anonymous_list_104 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_Group_25* u_Error_105 = &(group_u_Group_25) {
		.u_fields = u_anonymous_list_104,
	};
	
	group_u_Text_17* anonymous_string_literal_26 = &(group_u_Text_17) { .internal_value = "name" };
	
	group_u_Field_53* u_Parameter_name_27 = &(group_u_Field_53) {
		.u_value = u_anonymous_object_8,
		.u_name = anonymous_string_literal_26,
	};
	
	group_u_List_55* u_anonymous_list_93 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_List_55* u_anonymous_list_92 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_List_55* u_anonymous_list_94 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_Function_45* anonymous_function_95 = &(group_u_Function_45) {
		.u_return_type = u_Anything_19,
		.u_parameters = u_anonymous_list_93,
		.u_compile_time_parameters = u_anonymous_list_92,
		.u_tags = u_anonymous_list_94,
		.u_this_object = u_anonymous_object_8,
		.call = &call_anonymous_function_95
	};
	
	group_u_List_55* u_anonymous_list_30 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_Group_25* u_Parameter_31 = &(group_u_Group_25) {
		.u_fields = u_anonymous_list_30,
	};
	
	group_u_Text_17* anonymous_string_literal_1 = &(group_u_Text_17) { .internal_value = "Number.minus" };
	
	group_u_BuiltinTag_71* anonymous_object_87 = &(group_u_BuiltinTag_71) {
		.u_internal_name = anonymous_string_literal_1,
	};
	
	group_u_List_55* u_anonymous_list_100 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_Group_25* u_Number_101 = &(group_u_Group_25) {
		.u_fields = u_anonymous_list_100,
	};
	
	group_u_List_55* u_anonymous_list_24 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_List_55* u_anonymous_list_52 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_Group_25* u_Field_53 = &(group_u_Group_25) {
		.u_fields = u_anonymous_list_52,
	};
	
	group_u_Text_17* anonymous_string_literal_22 = &(group_u_Text_17) { .internal_value = "fields" };
	
	group_u_Field_53* u_Group_fields_23 = &(group_u_Field_53) {
		.u_name = anonymous_string_literal_22,
		.u_value = u_anonymous_object_8,
	};
	
	group_u_Text_17* anonymous_string_literal_48 = &(group_u_Text_17) { .internal_value = "name" };
	
	group_u_List_55* u_anonymous_list_75 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_List_55* u_anonymous_list_66 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_List_55* u_anonymous_list_83 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_Text_17* anonymous_string_literal_50 = &(group_u_Text_17) { .internal_value = "value" };
	
	group_u_Field_53* u_Field_value_51 = &(group_u_Field_53) {
		.u_value = u_anonymous_object_8,
		.u_name = anonymous_string_literal_50,
	};
	
	group_u_Text_17* anonymous_string_literal_3 = &(group_u_Text_17) { .internal_value = "terminal.input" };
	
	group_u_List_55* u_anonymous_list_44 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_Group_25* u_Function_45 = &(group_u_Group_25) {
		.u_fields = u_anonymous_list_44,
	};
	
	group_u_Field_53* u_Field_name_49 = &(group_u_Field_53) {
		.u_name = anonymous_string_literal_48,
		.u_value = u_anonymous_object_8,
	};
	
	group_u_Text_17* anonymous_string_literal_96 = &(group_u_Text_17) { .internal_value = "plus" };
	
	group_u_List_55* u_anonymous_list_58 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_Group_25* u_Either_59 = &(group_u_Group_25) {
		.u_fields = u_anonymous_list_58,
	};
	
	group_u_Text_17* anonymous_string_literal_36 = &(group_u_Text_17) { .internal_value = "return_type" };
	
	type_u_cabin_only_46* u_cabin_only_46 = &(type_u_cabin_only_46) {
		.empty = '0'
	};
	
	group_u_List_55* u_anonymous_list_76 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_Text_17* anonymous_string_literal_34 = &(group_u_Text_17) { .internal_value = "parameters" };
	
	group_u_Text_17* anonymous_string_literal_28 = &(group_u_Text_17) { .internal_value = "type" };
	
	group_u_Field_53* u_Parameter_type_29 = &(group_u_Field_53) {
		.u_name = anonymous_string_literal_28,
		.u_value = u_anonymous_object_8,
	};
	
	group_u_Text_17* anonymous_string_literal_42 = &(group_u_Text_17) { .internal_value = "this_object" };
	
	group_u_Field_53* u_Function_this_object_43 = &(group_u_Field_53) {
		.u_name = anonymous_string_literal_42,
		.u_value = u_anonymous_object_8,
	};
	
	group_u_Text_17* anonymous_string_literal_90 = &(group_u_Text_17) { .internal_value = "other" };
	
	group_u_Text_17* anonymous_string_literal_88 = &(group_u_Text_17) { .internal_value = "this" };
	
	group_u_Field_53* u_Either_variants_57 = &(group_u_Field_53) {
		.u_name = anonymous_string_literal_56,
		.u_value = u_anonymous_object_8,
	};
	
	group_u_Text_17* anonymous_string_literal_68 = &(group_u_Text_17) { .internal_value = "internal_name" };
	
	group_u_Field_53* u_BuiltinTag_internal_name_69 = &(group_u_Field_53) {
		.u_value = u_anonymous_object_8,
		.u_name = anonymous_string_literal_68,
	};
	
	group_u_List_55* u_anonymous_list_85 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_List_55* u_anonymous_list_84 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_Function_45* anonymous_function_86 = &(group_u_Function_45) {
		.u_compile_time_parameters = u_anonymous_list_83,
		.u_return_type = u_Anything_19,
		.u_tags = u_anonymous_list_85,
		.u_this_object = u_anonymous_object_8,
		.u_parameters = u_anonymous_list_84,
		.call = &call_anonymous_function_86
	};
	
	group_u_Field_53* u_Number_plus_97 = &(group_u_Field_53) {
		.u_name = anonymous_string_literal_96,
		.u_value = anonymous_function_86,
	};
	
	group_u_Text_17* anonymous_string_literal_40 = &(group_u_Text_17) { .internal_value = "tags" };
	
	group_u_Text_17* anonymous_string_literal_107 = &(group_u_Text_17) { .internal_value = "Data" };
	
	group_u_Text_17* anonymous_string_literal_38 = &(group_u_Text_17) { .internal_value = "compile_time_parameters" };
	
	group_u_Field_53* u_Function_compile_time_parameters_39 = &(group_u_Field_53) {
		.u_name = anonymous_string_literal_38,
		.u_value = u_anonymous_object_8,
	};
	
	group_u_Text_17* anonymous_string_literal_81 = &(group_u_Text_17) { .internal_value = "other" };
	
	group_u_Parameter_31* u_anonymous_function_other_82 = &(group_u_Parameter_31) {
		.u_type = u_Anything_19,
		.u_name = anonymous_string_literal_81,
	};
	
	group_u_List_55* u_anonymous_list_54 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_Group_25* u_List_55 = &(group_u_Group_25) {
		.u_fields = u_anonymous_list_54,
	};
	
	group_u_List_55* u_anonymous_list_32 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_Group_25* u_OneOf_33 = &(group_u_Group_25) {
		.u_fields = u_anonymous_list_32,
	};
	
	group_u_Text_17* anonymous_string_literal_111 = &(group_u_Text_17) { .internal_value = "object" };
	
	group_u_Parameter_31* u_anonymous_function_object_112 = &(group_u_Parameter_31) {
		.u_name = anonymous_string_literal_111,
		.u_type = u_Anything_19,
	};
	
	group_u_List_55* u_anonymous_list_70 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_Group_25* u_BuiltinTag_71 = &(group_u_Group_25) {
		.u_fields = u_anonymous_list_70,
	};
	
	group_u_Group_25* u_Group_25 = &(group_u_Group_25) {
		.u_fields = u_anonymous_list_24,
	};
	
	group_u_Text_17* anonymous_string_literal_4 = &(group_u_Text_17) { .internal_value = "Hello world!" };
	
	group_u_Text_17* anonymous_string_literal_98 = &(group_u_Text_17) { .internal_value = "minus" };
	
	group_u_Parameter_31* u_anonymous_function_other_91 = &(group_u_Parameter_31) {
		.u_type = u_Anything_19,
		.u_name = anonymous_string_literal_90,
	};
	
	group_u_List_55* u_anonymous_list_10 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_Field_53* u_Function_return_type_37 = &(group_u_Field_53) {
		.u_name = anonymous_string_literal_36,
		.u_value = u_anonymous_object_8,
	};
	
	group_u_Field_53* u_Number_minus_99 = &(group_u_Field_53) {
		.u_name = anonymous_string_literal_98,
		.u_value = anonymous_function_95,
	};
	
	group_u_BuiltinTag_71* anonymous_object_78 = &(group_u_BuiltinTag_71) {
		.u_internal_name = anonymous_string_literal_0,
	};
	
	group_u_List_55* u_anonymous_list_14 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_Field_53* u_Function_tags_41 = &(group_u_Field_53) {
		.u_name = anonymous_string_literal_40,
		.u_value = u_anonymous_object_8,
	};
	
	group_u_Field_53* u_Function_parameters_35 = &(group_u_Field_53) {
		.u_name = anonymous_string_literal_34,
		.u_value = u_anonymous_object_8,
	};
	
	group_u_BuiltinTag_71* anonymous_object_117 = &(group_u_BuiltinTag_71) {
		.u_internal_name = anonymous_string_literal_3,
	};
	
	group_u_Parameter_31* u_anonymous_function_this_89 = &(group_u_Parameter_31) {
		.u_name = anonymous_string_literal_88,
		.u_type = u_Anything_19,
	};
	
	group_u_List_55* u_anonymous_list_12 = &(group_u_List_55) {
		.empty = '0'
	};
	
	type_u_system_side_effects_47* u_system_side_effects_47 = &(type_u_system_side_effects_47) {
		.empty = '0'
	};
	
	group_u_Group_25* u_Object_21 = &(group_u_Group_25) {
		.u_fields = u_anonymous_list_20,
	};
	
	group_u_List_55* u_anonymous_list_106 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_List_55* u_anonymous_list_74 = &(group_u_List_55) {
		.empty = '0'
	};
	
	group_u_Text_17* anonymous_string_literal_13 = &(group_u_Text_17) { .internal_value = "Data" };
	
	(((void (*)(group_u_Group_25*))(u_terminal_122->u_print->call))(anonymous_string_literal_4));

	return 0;
}