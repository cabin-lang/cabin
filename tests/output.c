#include <stdio.h>
#include <stdlib.h>

typedef enum either_u_Boolean_67 u_Boolean_67;typedef struct u_Group_25 u_Group_25;
typedef struct type_u_cabin_only_46 type_u_cabin_only_46;
typedef struct u_This_6 u_This_6;
typedef struct u_Parameter_31 u_Parameter_31;
typedef struct type_u_system_side_effects_47 type_u_system_side_effects_47;
typedef struct u_Anything_19 u_Anything_19;
typedef struct u_Object_21 u_Object_21;
typedef struct type_u_terminal_122 type_u_terminal_122;
typedef struct u_Function_45 u_Function_45;
typedef struct u_BuiltinTag_71 u_BuiltinTag_71;
typedef struct u_Field_53 u_Field_53;
typedef struct u_Either_59 u_Either_59;
typedef struct type_u_anonymous_object_61 type_u_anonymous_object_61;
typedef struct u_List_55 u_List_55;
typedef struct u_Number_101 u_Number_101;
typedef enum either_u_Nothing_11 u_Nothing_11;typedef struct u_Text_17 u_Text_17;
typedef struct type_u_anonymous_object_64 type_u_anonymous_object_64;
typedef struct u_OneOf_33 u_OneOf_33;
typedef struct type_u_anonymous_object_8 type_u_anonymous_object_8;
typedef struct u_Error_105 u_Error_105;

enum u_Boolean {

	u_true,
	u_false,
};



struct u_Group_25 {
	void* u_fields;
};

struct type_u_cabin_only_46 {
	char empty;
};

struct u_This_6 {
	char empty;
};

struct u_Parameter_31 {
	void* u_name;
	void* u_type;
};

struct type_u_system_side_effects_47 {
	char empty;
};

struct u_Anything_19 {
	char empty;
};

struct u_Object_21 {
	char empty;
};

struct type_u_terminal_122 {
	u_Function_45* u_print;
	u_Function_45* u_input;
};

struct u_Function_45 {
	void* u_parameters;
	void* u_return_type;
	void* u_compile_time_parameters;
	void* u_tags;
	void* u_this_object;
	void* call;
};

struct u_BuiltinTag_71 {
	void* u_internal_name;
};

struct u_Field_53 {
	void* u_name;
	void* u_value;
};

struct u_Either_59 {
	void* u_variants;
};

struct type_u_anonymous_object_61 {
	char empty;
};

struct u_List_55 {
	char empty;
};

struct u_Number_101 {
	void* u_plus;
	void* u_minus;
};

enum u_Nothing {

	u_nothing,
};



struct u_Text_17 {
	char* internal_value;
};

struct type_u_anonymous_object_64 {
	char empty;
};

struct u_OneOf_33 {
	char empty;
};

struct type_u_anonymous_object_8 {
	char empty;
};

struct u_Error_105 {
	void* u_message;
};

void call_anonymous_function_121(u_Text_17* return_address) {
	char* buffer;
	size_t size;
	getline(&buffer, &size, stdin);
	*return_address = (u_Text_17) { .internal_value = buffer };
}

void call_anonymous_function_95(u_Anything_19* u_this, u_Anything_19* u_other, u_Anything_19* return_address) {

}

void call_anonymous_function_116(u_Anything_19* u_object) {
	printf("%s", u_object);
}

void call_anonymous_function_86(u_Anything_19* u_this, u_Anything_19* u_other, u_Anything_19* return_address) {

}

int main(int argc, char** argv) {
	u_List_55* u_anonymous_list_12 = &(u_List_55) {
		.empty = '0'
	};
	
	u_Text_17* anonymous_string_literal_22 = &(u_Text_17) { .internal_value = "fields" };
	
	u_List_55* u_anonymous_list_120 = &(u_List_55) {
		.empty = '0'
	};
	
	u_Text_17* anonymous_string_literal_102 = &(u_Text_17) { .internal_value = "message" };
	
	u_List_55* u_anonymous_list_10 = &(u_List_55) {
		.empty = '0'
	};
	
	u_List_55* u_anonymous_list_20 = &(u_List_55) {
		.empty = '0'
	};
	
	u_Text_17* anonymous_string_literal_68 = &(u_Text_17) { .internal_value = "internal_name" };
	
	u_List_55* u_anonymous_list_52 = &(u_List_55) {
		.empty = '0'
	};
	
	u_Text_17* anonymous_string_literal_3 = &(u_Text_17) { .internal_value = "terminal.input" };
	
	u_BuiltinTag_71* anonymous_object_117 = &(u_BuiltinTag_71) {
		.u_internal_name = anonymous_string_literal_3,
	};
	
	u_Text_17* anonymous_string_literal_28 = &(u_Text_17) { .internal_value = "type" };
	
	type_u_anonymous_object_8* u_anonymous_object_8 = &(type_u_anonymous_object_8) {
		.empty = '0'
	};
	
	u_Field_53* u_Parameter_type_29 = &(u_Field_53) {
		.u_name = anonymous_string_literal_28,
		.u_value = u_anonymous_object_8,
	};
	
	u_List_55* u_anonymous_list_66 = &(u_List_55) {
		.empty = '0'
	};
	
	u_Text_17* anonymous_string_literal_2 = &(u_Text_17) { .internal_value = "terminal.print" };
	
	u_BuiltinTag_71* anonymous_object_110 = &(u_BuiltinTag_71) {
		.u_internal_name = anonymous_string_literal_2,
	};
	
	u_Text_17* anonymous_string_literal_98 = &(u_Text_17) { .internal_value = "minus" };
	
	u_List_55* u_anonymous_list_24 = &(u_List_55) {
		.empty = '0'
	};
	
	u_Group_25* u_Group_25 = &(u_Group_25) {
		.u_fields = u_anonymous_list_24,
	};
	
	u_Text_17* anonymous_string_literal_26 = &(u_Text_17) { .internal_value = "name" };
	
	type_u_cabin_only_46* u_cabin_only_46 = &(type_u_cabin_only_46) {
		.empty = '0'
	};
	
	u_List_55* u_anonymous_list_58 = &(u_List_55) {
		.empty = '0'
	};
	
	type_u_anonymous_object_61* u_anonymous_object_61 = &(type_u_anonymous_object_61) {
		.empty = '0'
	};
	
	u_Text_17* anonymous_string_literal_60 = &(u_Text_17) { .internal_value = "true" };
	
	u_Field_53* u_true_62 = &(u_Field_53) {
		.u_value = u_anonymous_object_61,
		.u_name = anonymous_string_literal_60,
	};
	
	u_List_55* u_anonymous_list_93 = &(u_List_55) {
		.empty = '0'
	};
	
	u_Text_17* anonymous_string_literal_111 = &(u_Text_17) { .internal_value = "object" };
	
	u_List_55* u_anonymous_list_5 = &(u_List_55) {
		.empty = '0'
	};
	
	u_Group_25* u_This_6 = &(u_Group_25) {
		.u_fields = u_anonymous_list_5,
	};
	
	u_Text_17* anonymous_string_literal_40 = &(u_Text_17) { .internal_value = "tags" };
	
	u_Text_17* anonymous_string_literal_38 = &(u_Text_17) { .internal_value = "compile_time_parameters" };
	
	u_Field_53* u_Function_compile_time_parameters_39 = &(u_Field_53) {
		.u_name = anonymous_string_literal_38,
		.u_value = u_anonymous_object_8,
	};
	
	u_Text_17* anonymous_string_literal_1 = &(u_Text_17) { .internal_value = "Number.minus" };
	
	u_BuiltinTag_71* anonymous_object_87 = &(u_BuiltinTag_71) {
		.u_internal_name = anonymous_string_literal_1,
	};
	
	u_List_55* u_anonymous_list_30 = &(u_List_55) {
		.empty = '0'
	};
	
	u_Group_25* u_Parameter_31 = &(u_Group_25) {
		.u_fields = u_anonymous_list_30,
	};
	
	type_u_system_side_effects_47* u_system_side_effects_47 = &(type_u_system_side_effects_47) {
		.empty = '0'
	};
	
	u_List_55* u_anonymous_list_18 = &(u_List_55) {
		.empty = '0'
	};
	
	u_Group_25* u_Anything_19 = &(u_Group_25) {
		.u_fields = u_anonymous_list_18,
	};
	
	u_Text_17* anonymous_string_literal_34 = &(u_Text_17) { .internal_value = "parameters" };
	
	u_Text_17* anonymous_string_literal_42 = &(u_Text_17) { .internal_value = "this_object" };
	
	u_List_55* u_anonymous_list_83 = &(u_List_55) {
		.empty = '0'
	};
	
	u_List_55* u_anonymous_list_84 = &(u_List_55) {
		.empty = '0'
	};
	
	u_List_55* u_anonymous_list_85 = &(u_List_55) {
		.empty = '0'
	};
	
	u_Function_45* anonymous_function_86 = &(u_Function_45) {
		.u_compile_time_parameters = u_anonymous_list_83,
		.u_return_type = u_Anything_19,
		.u_this_object = u_anonymous_object_8,
		.u_parameters = u_anonymous_list_84,
		.u_tags = u_anonymous_list_85,
	};
	
	u_Text_17* anonymous_string_literal_96 = &(u_Text_17) { .internal_value = "plus" };
	
	u_Field_53* u_Number_plus_97 = &(u_Field_53) {
		.u_value = anonymous_function_86,
		.u_name = anonymous_string_literal_96,
	};
	
	u_Text_17* anonymous_string_literal_0 = &(u_Text_17) { .internal_value = "Number.plus" };
	
	u_BuiltinTag_71* anonymous_object_78 = &(u_BuiltinTag_71) {
		.u_internal_name = anonymous_string_literal_0,
	};
	
	u_List_55* u_anonymous_list_92 = &(u_List_55) {
		.empty = '0'
	};
	
	u_List_55* u_anonymous_list_74 = &(u_List_55) {
		.empty = '0'
	};
	
	u_List_55* u_anonymous_list_104 = &(u_List_55) {
		.empty = '0'
	};
	
	u_Group_25* u_Object_21 = &(u_Group_25) {
		.u_fields = u_anonymous_list_20,
	};
	
	u_List_55* u_anonymous_list_118 = &(u_List_55) {
		.empty = '0'
	};
	
	u_List_55* u_anonymous_list_16 = &(u_List_55) {
		.empty = '0'
	};
	
	u_Group_25* u_Text_17 = &(u_Group_25) {
		.u_fields = u_anonymous_list_16,
	};
	
	u_List_55* u_anonymous_list_119 = &(u_List_55) {
		.empty = '0'
	};
	
	u_Function_45* anonymous_function_121 = &(u_Function_45) {
		.u_this_object = u_anonymous_object_8,
		.u_tags = u_anonymous_list_120,
		.u_compile_time_parameters = u_anonymous_list_118,
		.u_return_type = u_Text_17,
		.u_parameters = u_anonymous_list_119,
	};
	
	u_List_55* u_anonymous_list_113 = &(u_List_55) {
		.empty = '0'
	};
	
	u_List_55* u_anonymous_list_114 = &(u_List_55) {
		.empty = '0'
	};
	
	u_List_55* u_anonymous_list_115 = &(u_List_55) {
		.empty = '0'
	};
	
	u_Function_45* anonymous_function_116 = &(u_Function_45) {
		.u_this_object = u_anonymous_object_8,
		.u_compile_time_parameters = u_anonymous_list_113,
		.u_parameters = u_anonymous_list_114,
		.u_tags = u_anonymous_list_115,
		.u_return_type = u_anonymous_object_8,
	};
	
	type_u_terminal_122* u_terminal_122 = &(type_u_terminal_122) {
		.u_print = anonymous_function_116,
		.u_input = anonymous_function_121,
	};
	
	u_List_55* u_anonymous_list_44 = &(u_List_55) {
		.empty = '0'
	};
	
	u_Group_25* u_Function_45 = &(u_Group_25) {
		.u_fields = u_anonymous_list_44,
	};
	
	u_Text_17* anonymous_string_literal_7 = &(u_Text_17) { .internal_value = "nothing" };
	
	u_Text_17* anonymous_string_literal_90 = &(u_Text_17) { .internal_value = "other" };
	
	u_Parameter_31* u_anonymous_function_other_91 = &(u_Parameter_31) {
		.u_type = u_Anything_19,
		.u_name = anonymous_string_literal_90,
	};
	
	u_Parameter_31* u_anonymous_function_object_112 = &(u_Parameter_31) {
		.u_name = anonymous_string_literal_111,
		.u_type = u_Anything_19,
	};
	
	u_List_55* u_anonymous_list_70 = &(u_List_55) {
		.empty = '0'
	};
	
	u_Group_25* u_BuiltinTag_71 = &(u_Group_25) {
		.u_fields = u_anonymous_list_70,
	};
	
	u_List_55* u_anonymous_list_94 = &(u_List_55) {
		.empty = '0'
	};
	
	u_Function_45* anonymous_function_95 = &(u_Function_45) {
		.u_return_type = u_Anything_19,
		.u_compile_time_parameters = u_anonymous_list_92,
		.u_tags = u_anonymous_list_94,
		.u_this_object = u_anonymous_object_8,
		.u_parameters = u_anonymous_list_93,
	};
	
	u_Field_53* u_Parameter_name_27 = &(u_Field_53) {
		.u_value = u_anonymous_object_8,
		.u_name = anonymous_string_literal_26,
	};
	
	u_List_55* u_anonymous_list_76 = &(u_List_55) {
		.empty = '0'
	};
	
	u_List_55* u_anonymous_list_75 = &(u_List_55) {
		.empty = '0'
	};
	
	u_Function_45* u_builtin_77 = &(u_Function_45) {
		.u_compile_time_parameters = u_anonymous_list_74,
		.u_tags = u_anonymous_list_76,
		.u_return_type = u_BuiltinTag_71,
		.u_parameters = u_anonymous_list_75,
		.u_this_object = u_anonymous_object_8,
	};
	
	u_Field_53* u_Group_fields_23 = &(u_Field_53) {
		.u_name = anonymous_string_literal_22,
		.u_value = u_anonymous_object_8,
	};
	
	u_Text_17* anonymous_string_literal_48 = &(u_Text_17) { .internal_value = "name" };
	
	u_Field_53* u_Function_tags_41 = &(u_Field_53) {
		.u_value = u_anonymous_object_8,
		.u_name = anonymous_string_literal_40,
	};
	
	u_Group_25* u_Field_53 = &(u_Group_25) {
		.u_fields = u_anonymous_list_52,
	};
	
	u_Text_17* anonymous_string_literal_72 = &(u_Text_17) { .internal_value = "name" };
	
	type_u_anonymous_object_64* u_anonymous_object_64 = &(type_u_anonymous_object_64) {
		.empty = '0'
	};
	
	u_Text_17* anonymous_string_literal_63 = &(u_Text_17) { .internal_value = "false" };
	
	u_Field_53* u_false_65 = &(u_Field_53) {
		.u_value = u_anonymous_object_64,
		.u_name = anonymous_string_literal_63,
	};
	
	u_Group_25* u_Either_59 = &(u_Group_25) {
		.u_fields = u_anonymous_list_58,
	};
	
	u_Field_53* u_Function_parameters_35 = &(u_Field_53) {
		.u_name = anonymous_string_literal_34,
		.u_value = u_anonymous_object_8,
	};
	
	u_Text_17* anonymous_string_literal_56 = &(u_Text_17) { .internal_value = "variants" };
	
	u_Field_53* u_Either_variants_57 = &(u_Field_53) {
		.u_value = u_anonymous_object_8,
		.u_name = anonymous_string_literal_56,
	};
	
	u_Text_17* anonymous_string_literal_36 = &(u_Text_17) { .internal_value = "return_type" };
	
	u_Field_53* u_Function_return_type_37 = &(u_Field_53) {
		.u_name = anonymous_string_literal_36,
		.u_value = u_anonymous_object_8,
	};
	
	u_Text_17* anonymous_string_literal_81 = &(u_Text_17) { .internal_value = "other" };
	
	u_List_55* u_anonymous_list_54 = &(u_List_55) {
		.empty = '0'
	};
	
	u_Group_25* u_List_55 = &(u_Group_25) {
		.u_fields = u_anonymous_list_54,
	};
	
	u_List_55* u_anonymous_list_100 = &(u_List_55) {
		.empty = '0'
	};
	
	u_Group_25* u_Number_101 = &(u_Group_25) {
		.u_fields = u_anonymous_list_100,
	};
	
	u_Parameter_31* u_anonymous_function_other_82 = &(u_Parameter_31) {
		.u_name = anonymous_string_literal_81,
		.u_type = u_Anything_19,
	};
	
	u_Field_53* u_Error_message_103 = &(u_Field_53) {
		.u_value = u_anonymous_object_8,
		.u_name = anonymous_string_literal_102,
	};
	
	u_Text_17* anonymous_string_literal_107 = &(u_Text_17) { .internal_value = "Data" };
	
	u_List_55* u_anonymous_list_14 = &(u_List_55) {
		.empty = '0'
	};
	
	u_List_55* u_anonymous_list_106 = &(u_List_55) {
		.empty = '0'
	};
	
	u_Text_17* anonymous_string_literal_50 = &(u_Text_17) { .internal_value = "value" };
	
	u_Field_53* u_Field_value_51 = &(u_Field_53) {
		.u_value = u_anonymous_object_8,
		.u_name = anonymous_string_literal_50,
	};
	
	u_Field_53* u_Function_this_object_43 = &(u_Field_53) {
		.u_value = u_anonymous_object_8,
		.u_name = anonymous_string_literal_42,
	};
	
	u_Text_17* anonymous_string_literal_79 = &(u_Text_17) { .internal_value = "this" };
	
	u_List_55* u_anonymous_list_32 = &(u_List_55) {
		.empty = '0'
	};
	
	u_Parameter_31* u_builtin_name_73 = &(u_Parameter_31) {
		.u_name = anonymous_string_literal_72,
		.u_type = u_Text_17,
	};
	
	u_Text_17* anonymous_string_literal_13 = &(u_Text_17) { .internal_value = "Data" };
	
	u_Field_53* u_BuiltinTag_internal_name_69 = &(u_Field_53) {
		.u_name = anonymous_string_literal_68,
		.u_value = u_anonymous_object_8,
	};
	
	u_Text_17* anonymous_string_literal_88 = &(u_Text_17) { .internal_value = "this" };
	
	u_Parameter_31* u_anonymous_function_this_80 = &(u_Parameter_31) {
		.u_type = u_Anything_19,
		.u_name = anonymous_string_literal_79,
	};
	
	u_Field_53* u_Field_name_49 = &(u_Field_53) {
		.u_value = u_anonymous_object_8,
		.u_name = anonymous_string_literal_48,
	};
	
	u_List_55* u_anonymous_list_108 = &(u_List_55) {
		.empty = '0'
	};
	
	u_Field_53* u_Number_minus_99 = &(u_Field_53) {
		.u_value = anonymous_function_95,
		.u_name = anonymous_string_literal_98,
	};
	
	u_Text_17* anonymous_string_literal_4 = &(u_Text_17) { .internal_value = "Hello world!" };
	
	u_Group_25* u_OneOf_33 = &(u_Group_25) {
		.u_fields = u_anonymous_list_32,
	};
	
	u_Group_25* u_Error_105 = &(u_Group_25) {
		.u_fields = u_anonymous_list_104,
	};
	
	u_Field_53* u_nothing_9 = &(u_Field_53) {
		.u_name = anonymous_string_literal_7,
		.u_value = u_anonymous_object_8,
	};
	
	u_Parameter_31* u_anonymous_function_this_89 = &(u_Parameter_31) {
		.u_name = anonymous_string_literal_88,
		.u_type = u_Anything_19,
	};
	
	(((void (*)(u_Anything_19*))(u_terminal_122->u_print->call))(anonymous_string_literal_4));

	return 0;
}