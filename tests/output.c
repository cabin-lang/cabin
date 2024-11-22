#include <stdio.h>
#include <stdlib.h>

typedef struct group_u_Text_16 group_u_Text_16;
typedef struct group_u_OneOf_32 group_u_OneOf_32;
typedef struct group_u_List_54 group_u_List_54;
typedef struct group_u_Object_20 group_u_Object_20;
typedef struct type_u_terminal_121 type_u_terminal_121;
typedef struct group_u_Anything_18 group_u_Anything_18;
typedef struct type_u_cabin_only_45 type_u_cabin_only_45;
typedef struct group_u_BuiltinTag_70 group_u_BuiltinTag_70;
typedef enum either_u_Nothing_10 u_Nothing_10;typedef struct group_u_Group_24 group_u_Group_24;
typedef struct group_u_Parameter_30 group_u_Parameter_30;
typedef struct type_u_anonymous_object_63 type_u_anonymous_object_63;
typedef struct type_u_system_side_effects_46 type_u_system_side_effects_46;
typedef struct group_u_Field_52 group_u_Field_52;
typedef struct group_u_This_5 group_u_This_5;
typedef struct group_u_Either_58 group_u_Either_58;
typedef struct group_u_Error_104 group_u_Error_104;
typedef enum either_u_Boolean_66 u_Boolean_66;typedef struct type_u_anonymous_object_7 type_u_anonymous_object_7;
typedef struct group_u_Number_100 group_u_Number_100;
typedef struct group_u_Function_44 group_u_Function_44;
typedef struct type_u_anonymous_object_60 type_u_anonymous_object_60;

struct group_u_Text_16 {
	char* internal_value;
	char empty;
};

struct group_u_OneOf_32 {
	char empty;
};

struct group_u_List_54 {
	char empty;
};

struct group_u_Object_20 {
	char empty;
};

struct type_u_terminal_121 {
	group_u_Function_44* u_input;
	group_u_Function_44* u_print;
};

struct group_u_Anything_18 {
	char empty;
};

struct type_u_cabin_only_45 {
	char empty;
};

struct group_u_BuiltinTag_70 {
	void* u_internal_name;
};

enum u_Nothing {

	u_nothing,
};

struct group_u_Group_24 {
	void* u_fields;
};

struct group_u_Parameter_30 {
	void* u_name;
	void* u_type;
};

struct type_u_anonymous_object_63 {
	char empty;
};

struct type_u_system_side_effects_46 {
	char empty;
};

struct group_u_Field_52 {
	void* u_name;
	void* u_value;
};

struct group_u_This_5 {
	char empty;
};

struct group_u_Either_58 {
	void* u_variants;
};

struct group_u_Error_104 {
	void* u_message;
};

enum u_Boolean {

	u_true,
	u_false,
};

struct type_u_anonymous_object_7 {
	char empty;
};

struct group_u_Number_100 {
	void* u_plus;
	void* u_minus;
	float internal_value;
};

struct group_u_Function_44 {
	void* u_parameters;
	void* u_return_type;
	void* u_compile_time_parameters;
	void* u_tags;
	void* u_this_object;
	void* call;
};

struct type_u_anonymous_object_60 {
	char empty;
};

void call_anonymous_function_115(group_u_Anything_18* u_object) {
	printf("%s\n", ((group_u_Text_16*) u_object)->internal_value);
}

void call_anonymous_function_85(group_u_Anything_18* u_this, group_u_Anything_18* u_other, group_u_Anything_18* return_address) {
	*((group_u_Anything_18*) return_address) = (group_u_Number_100) { .internal_value = ((group_u_Number_100*) u_this)->internal_value + ((group_u_Number_100*) u_other)->internal_value };
}

void call_anonymous_function_94(group_u_Anything_18* u_this, group_u_Anything_18* u_other, group_u_Anything_18* return_address) {

}

void call_anonymous_function_136(group_u_Anything_18* u_this, group_u_Anything_18* u_other, group_u_Anything_18* return_address) {
	*((group_u_Anything_18*) return_address) = (group_u_Number_100) { .internal_value = ((group_u_Number_100*) u_this)->internal_value + ((group_u_Number_100*) u_other)->internal_value };
}

void call_anonymous_function_120(group_u_Text_16* return_address) {
	char* buffer = malloc(sizeof(char) * 256);
	fgets(buffer, 256, stdin);
	*return_address = (group_u_Text_16) { .internal_value = buffer };
}

void call_anonymous_function_127(group_u_Anything_18* u_object) {
	printf("%s\n", ((group_u_Text_16*) u_object)->internal_value);
}

int main(int argc, char** argv) {
	group_u_List_54* u_anonymous_list_15 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_Group_24* u_Text_16 = &(group_u_Group_24) {
		.u_fields = u_anonymous_list_15,
	};
	
	group_u_Text_16* anonymous_string_literal_21 = &(group_u_Text_16) { .internal_value = "fields" };
	
	group_u_List_54* u_anonymous_list_53 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_List_54* u_anonymous_list_73 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_List_54* u_anonymous_list_99 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_List_54* u_anonymous_list_124 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_List_54* u_anonymous_list_17 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_Text_16* anonymous_string_literal_71 = &(group_u_Text_16) { .internal_value = "name" };
	
	group_u_Parameter_30* u_builtin_name_72 = &(group_u_Parameter_30) {
		.u_name = anonymous_string_literal_71,
		.u_type = u_Text_16,
	};
	
	group_u_Text_16* anonymous_string_literal_3 = &(group_u_Text_16) { .internal_value = "terminal.input" };
	
	group_u_BuiltinTag_70* anonymous_object_116 = &(group_u_BuiltinTag_70) {
		.u_internal_name = anonymous_string_literal_3,
	};
	
	group_u_Text_16* anonymous_string_literal_37 = &(group_u_Text_16) { .internal_value = "compile_time_parameters" };
	
	type_u_anonymous_object_7* u_anonymous_object_7 = &(type_u_anonymous_object_7) {
		.empty = '0'
	};
	
	group_u_Field_52* u_Function_compile_time_parameters_38 = &(group_u_Field_52) {
		.u_name = anonymous_string_literal_37,
		.u_value = u_anonymous_object_7,
	};
	
	group_u_Text_16* anonymous_string_literal_27 = &(group_u_Text_16) { .internal_value = "type" };
	
	group_u_List_54* u_anonymous_list_105 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_List_54* u_anonymous_list_31 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_Group_24* u_OneOf_32 = &(group_u_Group_24) {
		.u_fields = u_anonymous_list_31,
	};
	
	group_u_List_54* u_anonymous_list_65 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_Text_16* anonymous_string_literal_41 = &(group_u_Text_16) { .internal_value = "this_object" };
	
	group_u_Field_52* u_Function_this_object_42 = &(group_u_Field_52) {
		.u_name = anonymous_string_literal_41,
		.u_value = u_anonymous_object_7,
	};
	
	group_u_List_54* u_anonymous_list_113 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_List_54* u_anonymous_list_114 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_List_54* u_anonymous_list_112 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_Function_44* anonymous_function_115 = &(group_u_Function_44) {
		.u_parameters = u_anonymous_list_113,
		.u_tags = u_anonymous_list_114,
		.u_this_object = u_anonymous_object_7,
		.u_compile_time_parameters = u_anonymous_list_112,
		.u_return_type = u_anonymous_object_7,
		.call = &call_anonymous_function_115
	};
	
	group_u_Text_16* anonymous_string_literal_0 = &(group_u_Text_16) { .internal_value = "Number.plus" };
	
	group_u_BuiltinTag_70* anonymous_object_77 = &(group_u_BuiltinTag_70) {
		.u_internal_name = anonymous_string_literal_0,
	};
	
	group_u_List_54* u_anonymous_list_126 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_Text_16* anonymous_string_literal_2 = &(group_u_Text_16) { .internal_value = "terminal.print" };
	
	group_u_Text_16* anonymous_string_literal_129 = &(group_u_Text_16) { .internal_value = "this" };
	
	group_u_Group_24* u_Anything_18 = &(group_u_Group_24) {
		.u_fields = u_anonymous_list_17,
	};
	
	group_u_Parameter_30* u_anonymous_function_this_130 = &(group_u_Parameter_30) {
		.u_name = anonymous_string_literal_129,
		.u_type = u_Anything_18,
	};
	
	group_u_Text_16* anonymous_string_literal_78 = &(group_u_Text_16) { .internal_value = "this" };
	
	group_u_Parameter_30* u_anonymous_function_this_79 = &(group_u_Parameter_30) {
		.u_type = u_Anything_18,
		.u_name = anonymous_string_literal_78,
	};
	
	group_u_List_54* u_anonymous_list_69 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_Text_16* anonymous_string_literal_131 = &(group_u_Text_16) { .internal_value = "other" };
	
	group_u_Parameter_30* u_anonymous_function_other_132 = &(group_u_Parameter_30) {
		.u_type = u_Anything_18,
		.u_name = anonymous_string_literal_131,
	};
	
	group_u_BuiltinTag_70* anonymous_object_109 = &(group_u_BuiltinTag_70) {
		.u_internal_name = anonymous_string_literal_2,
	};
	
	group_u_Text_16* anonymous_string_literal_122 = &(group_u_Text_16) { .internal_value = "object" };
	
	group_u_Text_16* anonymous_string_literal_12 = &(group_u_Text_16) { .internal_value = "Data" };
	
	group_u_Group_24* u_List_54 = &(group_u_Group_24) {
		.u_fields = u_anonymous_list_53,
	};
	
	group_u_Text_16* anonymous_string_literal_49 = &(group_u_Text_16) { .internal_value = "value" };
	
	group_u_Field_52* u_Field_value_50 = &(group_u_Field_52) {
		.u_name = anonymous_string_literal_49,
		.u_value = u_anonymous_object_7,
	};
	
	group_u_List_54* u_anonymous_list_117 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_Text_16* anonymous_string_literal_101 = &(group_u_Text_16) { .internal_value = "message" };
	
	group_u_Field_52* u_Error_message_102 = &(group_u_Field_52) {
		.u_value = u_anonymous_object_7,
		.u_name = anonymous_string_literal_101,
	};
	
	group_u_List_54* u_anonymous_list_93 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_Text_16* anonymous_string_literal_59 = &(group_u_Text_16) { .internal_value = "true" };
	
	group_u_List_54* u_anonymous_list_133 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_List_54* u_anonymous_list_19 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_Group_24* u_Object_20 = &(group_u_Group_24) {
		.u_fields = u_anonymous_list_19,
	};
	
	group_u_Text_16* anonymous_string_literal_35 = &(group_u_Text_16) { .internal_value = "return_type" };
	
	group_u_Text_16* anonymous_string_literal_95 = &(group_u_Text_16) { .internal_value = "plus" };
	
	group_u_Field_52* u_Group_fields_22 = &(group_u_Field_52) {
		.u_name = anonymous_string_literal_21,
		.u_value = u_anonymous_object_7,
	};
	
	group_u_List_54* u_anonymous_list_43 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_List_54* u_anonymous_list_103 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_List_54* u_anonymous_list_118 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_List_54* u_anonymous_list_4 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_Text_16* anonymous_string_literal_6 = &(group_u_Text_16) { .internal_value = "nothing" };
	
	group_u_Text_16* anonymous_string_literal_47 = &(group_u_Text_16) { .internal_value = "name" };
	
	group_u_List_54* u_anonymous_list_84 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_List_54* u_anonymous_list_83 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_List_54* u_anonymous_list_82 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_Function_44* anonymous_function_85 = &(group_u_Function_44) {
		.u_return_type = u_Anything_18,
		.u_this_object = u_anonymous_object_7,
		.u_parameters = u_anonymous_list_83,
		.u_compile_time_parameters = u_anonymous_list_82,
		.u_tags = u_anonymous_list_84,
		.call = &call_anonymous_function_85
	};
	
	group_u_Text_16* anonymous_string_literal_25 = &(group_u_Text_16) { .internal_value = "name" };
	
	group_u_Field_52* u_Parameter_name_26 = &(group_u_Field_52) {
		.u_name = anonymous_string_literal_25,
		.u_value = u_anonymous_object_7,
	};
	
	group_u_Text_16* anonymous_string_literal_110 = &(group_u_Text_16) { .internal_value = "object" };
	
	group_u_List_54* u_anonymous_list_134 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_List_54* u_anonymous_list_119 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_Text_16* anonymous_string_literal_1 = &(group_u_Text_16) { .internal_value = "Number.minus" };
	
	group_u_BuiltinTag_70* anonymous_object_86 = &(group_u_BuiltinTag_70) {
		.u_internal_name = anonymous_string_literal_1,
	};
	
	group_u_Field_52* u_Number_plus_96 = &(group_u_Field_52) {
		.u_name = anonymous_string_literal_95,
		.u_value = anonymous_function_85,
	};
	
	group_u_List_54* u_anonymous_list_91 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_List_54* u_anonymous_list_13 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_List_54* u_anonymous_list_92 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_Function_44* anonymous_function_94 = &(group_u_Function_44) {
		.u_compile_time_parameters = u_anonymous_list_91,
		.u_this_object = u_anonymous_object_7,
		.u_return_type = u_Anything_18,
		.u_parameters = u_anonymous_list_92,
		.u_tags = u_anonymous_list_93,
		.call = &call_anonymous_function_94
	};
	
	group_u_Number_100* u_anonymous_number_137 = &(group_u_Number_100) { .internal_value = 4 };
	
	group_u_Function_44* anonymous_function_120 = &(group_u_Function_44) {
		.u_parameters = u_anonymous_list_118,
		.u_tags = u_anonymous_list_119,
		.u_return_type = u_Text_16,
		.u_this_object = u_anonymous_object_7,
		.u_compile_time_parameters = u_anonymous_list_117,
		.call = &call_anonymous_function_120
	};
	
	type_u_terminal_121* u_terminal_121 = &(type_u_terminal_121) {
		.u_input = anonymous_function_120,
		.u_print = anonymous_function_115,
	};
	
	group_u_Field_52* u_nothing_8 = &(group_u_Field_52) {
		.u_name = anonymous_string_literal_6,
		.u_value = u_anonymous_object_7,
	};
	
	group_u_Text_16* anonymous_string_literal_80 = &(group_u_Text_16) { .internal_value = "other" };
	
	group_u_Parameter_30* u_anonymous_function_other_81 = &(group_u_Parameter_30) {
		.u_type = u_Anything_18,
		.u_name = anonymous_string_literal_80,
	};
	
	group_u_Text_16* anonymous_string_literal_33 = &(group_u_Text_16) { .internal_value = "parameters" };
	
	group_u_Field_52* u_Function_parameters_34 = &(group_u_Field_52) {
		.u_name = anonymous_string_literal_33,
		.u_value = u_anonymous_object_7,
	};
	
	group_u_Number_100* u_anonymous_number_128 = &(group_u_Number_100) { .internal_value = 3 };
	
	group_u_List_54* u_anonymous_list_135 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_Text_16* anonymous_string_literal_67 = &(group_u_Text_16) { .internal_value = "internal_name" };
	
	group_u_Field_52* u_BuiltinTag_internal_name_68 = &(group_u_Field_52) {
		.u_name = anonymous_string_literal_67,
		.u_value = u_anonymous_object_7,
	};
	
	group_u_List_54* u_anonymous_list_74 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_Text_16* anonymous_string_literal_97 = &(group_u_Text_16) { .internal_value = "minus" };
	
	type_u_cabin_only_45* u_cabin_only_45 = &(type_u_cabin_only_45) {
		.empty = '0'
	};
	
	group_u_List_54* u_anonymous_list_51 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_Field_52* u_Number_minus_98 = &(group_u_Field_52) {
		.u_name = anonymous_string_literal_97,
		.u_value = anonymous_function_94,
	};
	
	group_u_Group_24* u_BuiltinTag_70 = &(group_u_Group_24) {
		.u_fields = u_anonymous_list_69,
	};
	
	group_u_List_54* u_anonymous_list_9 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_List_54* u_anonymous_list_125 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_Parameter_30* u_anonymous_function_object_123 = &(group_u_Parameter_30) {
		.u_name = anonymous_string_literal_122,
		.u_type = u_Anything_18,
	};
	
	group_u_Text_16* anonymous_string_literal_89 = &(group_u_Text_16) { .internal_value = "other" };
	
	group_u_List_54* u_anonymous_list_11 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_List_54* u_anonymous_list_23 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_Group_24* u_Group_24 = &(group_u_Group_24) {
		.u_fields = u_anonymous_list_23,
	};
	
	group_u_List_54* u_anonymous_list_107 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_List_54* u_anonymous_list_29 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_Group_24* u_Parameter_30 = &(group_u_Group_24) {
		.u_fields = u_anonymous_list_29,
	};
	
	type_u_anonymous_object_63* u_anonymous_object_63 = &(type_u_anonymous_object_63) {
		.empty = '0'
	};
	
	group_u_Text_16* anonymous_string_literal_62 = &(group_u_Text_16) { .internal_value = "false" };
	
	group_u_Field_52* u_false_64 = &(group_u_Field_52) {
		.u_value = u_anonymous_object_63,
		.u_name = anonymous_string_literal_62,
	};
	
	group_u_Text_16* anonymous_string_literal_39 = &(group_u_Text_16) { .internal_value = "tags" };
	
	group_u_Field_52* u_Function_tags_40 = &(group_u_Field_52) {
		.u_name = anonymous_string_literal_39,
		.u_value = u_anonymous_object_7,
	};
	
	group_u_Field_52* u_Function_return_type_36 = &(group_u_Field_52) {
		.u_value = u_anonymous_object_7,
		.u_name = anonymous_string_literal_35,
	};
	
	type_u_system_side_effects_46* u_system_side_effects_46 = &(type_u_system_side_effects_46) {
		.empty = '0'
	};
	
	group_u_Parameter_30* u_anonymous_function_other_90 = &(group_u_Parameter_30) {
		.u_name = anonymous_string_literal_89,
		.u_type = u_Anything_18,
	};
	
	group_u_Function_44* anonymous_function_136 = &(group_u_Function_44) {
		.u_parameters = u_anonymous_list_134,
		.u_tags = u_anonymous_list_135,
		.u_this_object = u_anonymous_number_128,
		.u_compile_time_parameters = u_anonymous_list_133,
		.u_return_type = u_Anything_18,
		.call = &call_anonymous_function_136
	};
	
	group_u_List_54* u_anonymous_list_57 = &(group_u_List_54) {
		.empty = '0'
	};
	
	type_u_anonymous_object_60* u_anonymous_object_60 = &(type_u_anonymous_object_60) {
		.empty = '0'
	};
	
	group_u_Field_52* u_true_61 = &(group_u_Field_52) {
		.u_name = anonymous_string_literal_59,
		.u_value = u_anonymous_object_60,
	};
	
	group_u_Group_24* u_Field_52 = &(group_u_Group_24) {
		.u_fields = u_anonymous_list_51,
	};
	
	group_u_Group_24* u_This_5 = &(group_u_Group_24) {
		.u_fields = u_anonymous_list_4,
	};
	
	group_u_List_54* u_anonymous_list_75 = &(group_u_List_54) {
		.empty = '0'
	};
	
	group_u_Function_44* anonymous_function_127 = &(group_u_Function_44) {
		.u_return_type = u_anonymous_object_7,
		.u_this_object = u_terminal_121,
		.u_compile_time_parameters = u_anonymous_list_124,
		.u_parameters = u_anonymous_list_125,
		.u_tags = u_anonymous_list_126,
		.call = &call_anonymous_function_127
	};
	
	group_u_Group_24* u_Either_58 = &(group_u_Group_24) {
		.u_fields = u_anonymous_list_57,
	};
	
	group_u_Field_52* u_Field_name_48 = &(group_u_Field_52) {
		.u_value = u_anonymous_object_7,
		.u_name = anonymous_string_literal_47,
	};
	
	group_u_Text_16* anonymous_string_literal_55 = &(group_u_Text_16) { .internal_value = "variants" };
	
	group_u_Field_52* u_Either_variants_56 = &(group_u_Field_52) {
		.u_name = anonymous_string_literal_55,
		.u_value = u_anonymous_object_7,
	};
	
	group_u_Text_16* anonymous_string_literal_106 = &(group_u_Text_16) { .internal_value = "Data" };
	
	group_u_Group_24* u_Error_104 = &(group_u_Group_24) {
		.u_fields = u_anonymous_list_103,
	};
	
	group_u_Field_52* u_Parameter_type_28 = &(group_u_Field_52) {
		.u_value = u_anonymous_object_7,
		.u_name = anonymous_string_literal_27,
	};
	
	group_u_Text_16* anonymous_string_literal_87 = &(group_u_Text_16) { .internal_value = "this" };
	
	group_u_Parameter_30* u_anonymous_function_this_88 = &(group_u_Parameter_30) {
		.u_type = u_Anything_18,
		.u_name = anonymous_string_literal_87,
	};
	
	group_u_Group_24* u_Number_100 = &(group_u_Group_24) {
		.u_fields = u_anonymous_list_99,
	};
	
	group_u_Group_24* u_Function_44 = &(group_u_Group_24) {
		.u_fields = u_anonymous_list_43,
	};
	
	group_u_Parameter_30* u_anonymous_function_object_111 = &(group_u_Parameter_30) {
		.u_name = anonymous_string_literal_110,
		.u_type = u_Anything_18,
	};
	
				({
						
					void* arg0 = ({
		group_u_Anything_18* return_address;	
		void* arg0 = u_anonymous_number_137;
		(((void (*)(group_u_Group_24*, group_u_Group_24*))(anonymous_function_136->call))(u_anonymous_number_128, arg0, return_address));
		return_address;
	})	
	;
					(((void (*)(group_u_Group_24*))(anonymous_function_127->call))(arg0));
					
				})	
				;

	return 0;
}