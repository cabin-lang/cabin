#include <stdio.h>
#include <stdlib.h>

typedef struct type_u_anonymous_object_8 type_u_anonymous_object_8;
typedef struct group_u_Error_115 group_u_Error_115;
typedef struct group_u_Group_26 group_u_Group_26;
typedef struct type_u_terminal_135 type_u_terminal_135;
typedef struct group_u_Object_23 group_u_Object_23;
typedef enum either_u_Nothing_11 u_Nothing_11;
typedef struct group_u_List_63 group_u_List_63;
typedef struct group_u_Field_56 group_u_Field_56;
typedef enum either_u_Boolean_77 u_Boolean_77;
typedef struct group_u_This_5 group_u_This_5;
typedef struct type_u_anonymous_object_74 type_u_anonymous_object_74;
typedef struct group_u_Function_41 group_u_Function_41;
typedef struct group_u_Number_90 group_u_Number_90;
typedef struct type_u_cabin_only_53 type_u_cabin_only_53;
typedef struct group_u_Text_17 group_u_Text_17;
typedef struct group_u_BuiltinTag_79 group_u_BuiltinTag_79;
typedef struct group_u_Anything_20 group_u_Anything_20;
typedef struct group_u_OneOf_38 group_u_OneOf_38;
typedef struct group_u_Parameter_31 group_u_Parameter_31;
typedef struct group_u_Either_66 group_u_Either_66;
typedef struct type_u_anonymous_object_71 type_u_anonymous_object_71;
typedef struct type_u_system_side_effects_54 type_u_system_side_effects_54;

struct type_u_anonymous_object_8
{
	char empty;
};

struct group_u_Error_115
{
	void *u_message;
};

struct group_u_Group_26
{
	void *u_fields;
};

struct type_u_terminal_135
{
	group_u_Function_41 *u_print;
	group_u_Function_41 *u_input;
};

struct group_u_Object_23
{
	char empty;
};

enum u_Nothing
{

	u_nothing,
};

struct group_u_List_63
{
	char empty;
};

struct group_u_Field_56
{
	void *u_name;
	void *u_value;
};

enum u_Boolean
{

	u_true,
	u_false,
};

struct group_u_This_5
{
	char empty;
};

struct type_u_anonymous_object_74
{
	char empty;
};

struct group_u_Function_41
{
	void *u_parameters;
	void *u_return_type;
	void *u_compile_time_parameters;
	void *u_tags;
	void *u_this_object;
	void *call;
};

struct group_u_Number_90
{
	void *u_plus;
	void *u_minus;
	float internal_value;
};

struct type_u_cabin_only_53
{
	char empty;
};

struct group_u_Text_17
{
	char *internal_value;
	char empty;
};

struct group_u_BuiltinTag_79
{
	void *u_internal_name;
};

struct group_u_Anything_20
{
	char empty;
};

struct group_u_OneOf_38
{
	char empty;
};

struct group_u_Parameter_31
{
	void *u_name;
	void *u_type;
};

struct group_u_Either_66
{
	void *u_variants;
};

struct type_u_anonymous_object_71
{
	char empty;
};

struct type_u_system_side_effects_54
{
	char empty;
};

/*

struct AnythingWrapper {
	void* tag,
	AnythingUnion value
}


*/

void call_anonymous_function_134(group_u_Text_17 *return_address)
{
	char *buffer = malloc(sizeof(char) * 256);
	fgets(buffer, 256, stdin);
	*return_address = (group_u_Text_17){.internal_value = buffer};
}

void call_anonymous_function_99(group_u_Number_90 *u_this, group_u_Number_90 *u_other, group_u_Number_90 *return_address)
{
	*return_address = (group_u_Number_90){.internal_value = u_this->internal_value + u_other->internal_value};
}

void call_anonymous_function_129(group_u_Anything_20 *u_object)
{
	group_u_Text_17 *return_address;
	(((void (*)(group_u_Anything_20 *, group_u_Text_17 *))(u_object->to_string->call))(object, return_address));
	printf("%s\n", result->internal_value);
}

void call_anonymous_function_141(group_u_Anything_20 *u_object)
{
	group_u_Text_17 *return_address;
	(((void (*)(group_u_Anything_20 *, group_u_Text_17 *))(u_object->to_string->call))(object, return_address));
	printf("%s\n", result->internal_value);
}

void call_anonymous_function_150(group_u_Number_90 *u_this, group_u_Number_90 *u_other, group_u_Number_90 *return_address)
{
	*return_address = (group_u_Number_90){.internal_value = u_this->internal_value + u_other->internal_value};
}

void call_anonymous_function_108(group_u_Number_90 *u_this, group_u_Number_90 *u_other, group_u_Number_90 *return_address)
{
}

int main(int argc, char **argv)
{
	group_u_List_63 *u_anonymous_list_126 = &(group_u_List_63){
		.empty = '0'};

	group_u_Text_17 *anonymous_string_literal_32 = &(group_u_Text_17){.internal_value = "name"};

	group_u_List_63 *u_anonymous_list_96 = &(group_u_List_63){
		.empty = '0'};

	group_u_Text_17 *anonymous_string_literal_13 = &(group_u_Text_17){.internal_value = "Data"};

	group_u_List_63 *u_anonymous_list_21 = &(group_u_List_63){
		.empty = '0'};

	group_u_List_63 *u_anonymous_list_55 = &(group_u_List_63){
		.empty = '0'};

	group_u_List_63 *u_anonymous_list_22 = &(group_u_List_63){
		.empty = '0'};

	group_u_Object_23 *u_anonymous_object_8 = &(type_u_anonymous_object_8){
		.empty = '0'};

	group_u_List_63 *u_anonymous_list_132 = &(group_u_List_63){
		.empty = '0'};

	group_u_List_63 *u_anonymous_list_131 = &(group_u_List_63){
		.empty = '0'};

	group_u_List_63 *u_anonymous_list_133 = &(group_u_List_63){
		.empty = '0'};

	group_u_List_63 *u_anonymous_list_18 = &(group_u_List_63){
		.empty = '0'};

	group_u_Group_26 *u_Text_17 = &(group_u_Group_26){
		.u_fields = u_anonymous_list_18,
	};

	group_u_Function_41 *anonymous_function_134 = &(group_u_Function_41){
		.u_this_object = u_anonymous_object_8,
		.u_parameters = u_anonymous_list_132,
		.u_compile_time_parameters = u_anonymous_list_131,
		.u_tags = u_anonymous_list_133,
		.u_return_type = u_Text_17,
		.call = &call_anonymous_function_134};

	group_u_List_63 *u_anonymous_list_87 = &(group_u_List_63){
		.empty = '0'};

	group_u_List_63 *u_anonymous_list_65 = &(group_u_List_63){
		.empty = '0'};

	group_u_List_63 *u_anonymous_list_86 = &(group_u_List_63){
		.empty = '0'};

	group_u_List_63 *u_anonymous_list_148 = &(group_u_List_63){
		.empty = '0'};

	group_u_Text_17 *anonymous_string_literal_46 = &(group_u_Text_17){.internal_value = "compile_time_parameters"};

	group_u_Field_56 *u_Function_compile_time_parameters_47 = &(group_u_Field_56){
		.u_name = anonymous_string_literal_46,
		.u_value = u_anonymous_object_8,
	};

	group_u_List_63 *u_anonymous_list_97 = &(group_u_List_63){
		.empty = '0'};

	group_u_List_63 *u_anonymous_list_98 = &(group_u_List_63){
		.empty = '0'};

	group_u_List_63 *u_anonymous_list_113 = &(group_u_List_63){
		.empty = '0'};

	group_u_Group_26 *u_Number_90 = &(group_u_Group_26){
		.u_fields = u_anonymous_list_113,
	};

	group_u_Function_41 *anonymous_function_99 = &(group_u_Function_41){
		.u_parameters = u_anonymous_list_97,
		.u_tags = u_anonymous_list_98,
		.u_compile_time_parameters = u_anonymous_list_96,
		.u_this_object = u_anonymous_object_8,
		.u_return_type = u_Number_90,
		.call = &call_anonymous_function_99};

	group_u_Text_17 *anonymous_string_literal_145 = &(group_u_Text_17){.internal_value = "other"};

	group_u_List_63 *u_anonymous_list_12 = &(group_u_List_63){
		.empty = '0'};

	group_u_List_63 *u_anonymous_list_36 = &(group_u_List_63){
		.empty = '0'};

	group_u_Text_17 *anonymous_string_literal_143 = &(group_u_Text_17){.internal_value = "this"};

	group_u_Text_17 *anonymous_string_literal_109 = &(group_u_Text_17){.internal_value = "plus"};

	group_u_Field_56 *u_Number_plus_110 = &(group_u_Field_56){
		.u_value = anonymous_function_99,
		.u_name = anonymous_string_literal_109,
	};

	group_u_Text_17 *anonymous_string_literal_116 = &(group_u_Text_17){.internal_value = "message"};

	group_u_Field_56 *u_Error_message_117 = &(group_u_Field_56){
		.u_name = anonymous_string_literal_116,
		.u_value = u_anonymous_object_8,
	};

	group_u_Text_17 *anonymous_string_literal_70 = &(group_u_Text_17){.internal_value = "true"};

	group_u_Object_23 *u_anonymous_object_71 = &(type_u_anonymous_object_71){
		.empty = '0'};

	group_u_Field_56 *u_true_72 = &(group_u_Field_56){
		.u_name = anonymous_string_literal_70,
		.u_value = u_anonymous_object_71,
	};

	group_u_Field_56 *u_Parameter_name_33 = &(group_u_Field_56){
		.u_name = anonymous_string_literal_32,
		.u_value = u_anonymous_object_8,
	};

	group_u_Text_17 *anonymous_string_literal_59 = &(group_u_Text_17){.internal_value = "value"};

	group_u_Field_56 *u_Field_value_60 = &(group_u_Field_56){
		.u_name = anonymous_string_literal_59,
		.u_value = u_anonymous_object_8,
	};

	group_u_List_63 *u_anonymous_list_118 = &(group_u_List_63){
		.empty = '0'};

	group_u_Group_26 *u_Error_115 = &(group_u_Group_26){
		.u_fields = u_anonymous_list_118,
	};

	group_u_List_63 *u_anonymous_list_76 = &(group_u_List_63){
		.empty = '0'};

	group_u_List_63 *u_anonymous_list_119 = &(group_u_List_63){
		.empty = '0'};

	group_u_Text_17 *anonymous_string_literal_34 = &(group_u_Text_17){.internal_value = "type"};

	group_u_List_63 *u_anonymous_list_29 = &(group_u_List_63){
		.empty = '0'};

	group_u_Group_26 *u_Group_26 = &(group_u_Group_26){
		.u_fields = u_anonymous_list_29,
	};

	group_u_List_63 *u_anonymous_list_85 = &(group_u_List_63){
		.empty = '0'};

	group_u_List_63 *u_anonymous_list_138 = &(group_u_List_63){
		.empty = '0'};

	group_u_Text_17 *anonymous_string_literal_80 = &(group_u_Text_17){.internal_value = "internal_name"};

	group_u_Field_56 *u_BuiltinTag_internal_name_81 = &(group_u_Field_56){
		.u_name = anonymous_string_literal_80,
		.u_value = u_anonymous_object_8,
	};

	group_u_List_63 *u_anonymous_list_105 = &(group_u_List_63){
		.empty = '0'};

	group_u_List_63 *u_anonymous_list_127 = &(group_u_List_63){
		.empty = '0'};

	group_u_List_63 *u_anonymous_list_128 = &(group_u_List_63){
		.empty = '0'};

	group_u_Function_41 *anonymous_function_129 = &(group_u_Function_41){
		.u_parameters = u_anonymous_list_127,
		.u_return_type = u_anonymous_object_8,
		.u_this_object = u_anonymous_object_8,
		.u_tags = u_anonymous_list_128,
		.u_compile_time_parameters = u_anonymous_list_126,
		.call = &call_anonymous_function_129};

	group_u_Object_23 *u_terminal_135 = &(type_u_terminal_135){
		.u_print = anonymous_function_129,
		.u_input = anonymous_function_134,
	};

	group_u_List_63 *u_anonymous_list_24 = &(group_u_List_63){
		.empty = '0'};

	group_u_Group_26 *u_Object_23 = &(group_u_Group_26){
		.u_fields = u_anonymous_list_24,
	};

	group_u_List_63 *u_anonymous_list_106 = &(group_u_List_63){
		.empty = '0'};

	group_u_List_63 *u_anonymous_list_107 = &(group_u_List_63){
		.empty = '0'};

	group_u_Function_41 *anonymous_function_108 = &(group_u_Function_41){
		.u_parameters = u_anonymous_list_106,
		.u_compile_time_parameters = u_anonymous_list_105,
		.u_return_type = u_Number_90,
		.u_this_object = u_anonymous_object_8,
		.u_tags = u_anonymous_list_107,
		.call = &call_anonymous_function_108};

	group_u_Number_90 *u_anonymous_number_151 = &(group_u_Number_90){.internal_value = 4};

	group_u_List_63 *u_anonymous_list_19 = &(group_u_List_63){
		.empty = '0'};

	group_u_Text_17 *anonymous_string_literal_27 = &(group_u_Text_17){.internal_value = "fields"};

	group_u_Field_56 *u_Group_fields_28 = &(group_u_Field_56){
		.u_name = anonymous_string_literal_27,
		.u_value = u_anonymous_object_8,
	};

	group_u_List_63 *u_anonymous_list_64 = &(group_u_List_63){
		.empty = '0'};

	group_u_Group_26 *u_List_63 = &(group_u_Group_26){
		.u_fields = u_anonymous_list_64,
	};

	group_u_Text_17 *anonymous_string_literal_67 = &(group_u_Text_17){.internal_value = "variants"};

	group_u_Field_56 *u_Either_variants_68 = &(group_u_Field_56){
		.u_value = u_anonymous_object_8,
		.u_name = anonymous_string_literal_67,
	};

	group_u_Text_17 *anonymous_string_literal_103 = &(group_u_Text_17){.internal_value = "other"};

	group_u_Text_17 *anonymous_string_literal_57 = &(group_u_Text_17){.internal_value = "name"};

	group_u_Text_17 *anonymous_string_literal_83 = &(group_u_Text_17){.internal_value = "name"};

	group_u_Text_17 *anonymous_string_literal_3 = &(group_u_Text_17){.internal_value = "terminal.input"};

	group_u_Text_17 *anonymous_string_literal_48 = &(group_u_Text_17){.internal_value = "tags"};

	group_u_Field_56 *u_Function_tags_49 = &(group_u_Field_56){
		.u_name = anonymous_string_literal_48,
		.u_value = u_anonymous_object_8,
	};

	group_u_List_63 *u_anonymous_list_61 = &(group_u_List_63){
		.empty = '0'};

	group_u_Group_26 *u_Field_56 = &(group_u_Group_26){
		.u_fields = u_anonymous_list_61,
	};

	group_u_List_63 *u_anonymous_list_14 = &(group_u_List_63){
		.empty = '0'};

	group_u_BuiltinTag_79 *anonymous_object_130 = &(group_u_BuiltinTag_79){
		.u_internal_name = anonymous_string_literal_3,
	};

	group_u_List_63 *u_anonymous_list_139 = &(group_u_List_63){
		.empty = '0'};

	group_u_List_63 *u_anonymous_list_16 = &(group_u_List_63){
		.empty = '0'};

	group_u_Text_17 *anonymous_string_literal_136 = &(group_u_Text_17){.internal_value = "object"};

	group_u_Parameter_31 *u_builtin_name_84 = &(group_u_Parameter_31){
		.u_type = u_Text_17,
		.u_name = anonymous_string_literal_83,
	};

	group_u_Text_17 *anonymous_string_literal_124 = &(group_u_Text_17){.internal_value = "object"};

	group_u_Text_17 *anonymous_string_literal_0 = &(group_u_Text_17){.internal_value = "Number.plus"};

	group_u_Text_17 *anonymous_string_literal_44 = &(group_u_Text_17){.internal_value = "return_type"};

	group_u_Field_56 *u_Function_return_type_45 = &(group_u_Field_56){
		.u_value = u_anonymous_object_8,
		.u_name = anonymous_string_literal_44,
	};

	group_u_Field_56 *u_Field_name_58 = &(group_u_Field_56){
		.u_name = anonymous_string_literal_57,
		.u_value = u_anonymous_object_8,
	};

	group_u_Text_17 *anonymous_string_literal_92 = &(group_u_Text_17){.internal_value = "this"};

	group_u_List_63 *u_anonymous_list_30 = &(group_u_List_63){
		.empty = '0'};

	group_u_List_63 *u_anonymous_list_6 = &(group_u_List_63){
		.empty = '0'};

	group_u_Group_26 *u_This_5 = &(group_u_Group_26){
		.u_fields = u_anonymous_list_6,
	};

	group_u_List_63 *u_anonymous_list_37 = &(group_u_List_63){
		.empty = '0'};

	group_u_List_63 *u_anonymous_list_78 = &(group_u_List_63){
		.empty = '0'};

	group_u_List_63 *u_anonymous_list_89 = &(group_u_List_63){
		.empty = '0'};

	group_u_Object_23 *u_anonymous_object_74 = &(type_u_anonymous_object_74){
		.empty = '0'};

	group_u_List_63 *u_anonymous_list_52 = &(group_u_List_63){
		.empty = '0'};

	group_u_Group_26 *u_Function_41 = &(group_u_Group_26){
		.u_fields = u_anonymous_list_52,
	};

	group_u_List_63 *u_anonymous_list_40 = &(group_u_List_63){
		.empty = '0'};

	group_u_List_63 *u_anonymous_list_82 = &(group_u_List_63){
		.empty = '0'};

	group_u_Text_17 *anonymous_string_literal_111 = &(group_u_Text_17){.internal_value = "minus"};

	group_u_Text_17 *anonymous_string_literal_2 = &(group_u_Text_17){.internal_value = "terminal.print"};

	group_u_Text_17 *anonymous_string_literal_7 = &(group_u_Text_17){.internal_value = "nothing"};

	group_u_Parameter_31 *u_anonymous_function_this_144 = &(group_u_Parameter_31){
		.u_type = u_Number_90,
		.u_name = anonymous_string_literal_143,
	};

	group_u_List_63 *u_anonymous_list_4 = &(group_u_List_63){
		.empty = '0'};

	group_u_Field_56 *u_Parameter_type_35 = &(group_u_Field_56){
		.u_name = anonymous_string_literal_34,
		.u_value = u_anonymous_object_8,
	};

	group_u_Text_17 *anonymous_string_literal_73 = &(group_u_Text_17){.internal_value = "false"};

	group_u_List_63 *u_anonymous_list_114 = &(group_u_List_63){
		.empty = '0'};

	group_u_Object_23 *u_cabin_only_53 = &(type_u_cabin_only_53){
		.empty = '0'};

	group_u_Text_17 *anonymous_string_literal_94 = &(group_u_Text_17){.internal_value = "other"};

	group_u_Parameter_31 *u_anonymous_function_other_95 = &(group_u_Parameter_31){
		.u_name = anonymous_string_literal_94,
		.u_type = u_Number_90,
	};

	group_u_Field_56 *u_Number_minus_112 = &(group_u_Field_56){
		.u_name = anonymous_string_literal_111,
		.u_value = anonymous_function_108,
	};

	group_u_Group_26 *u_BuiltinTag_79 = &(group_u_Group_26){
		.u_fields = u_anonymous_list_82,
	};

	group_u_Text_17 *anonymous_string_literal_50 = &(group_u_Text_17){.internal_value = "this_object"};

	group_u_Text_17 *anonymous_string_literal_120 = &(group_u_Text_17){.internal_value = "Data"};

	group_u_List_63 *u_anonymous_list_140 = &(group_u_List_63){
		.empty = '0'};

	group_u_Function_41 *anonymous_function_141 = &(group_u_Function_41){
		.u_this_object = u_terminal_135,
		.u_parameters = u_anonymous_list_139,
		.u_compile_time_parameters = u_anonymous_list_138,
		.u_return_type = u_anonymous_object_8,
		.u_tags = u_anonymous_list_140,
		.call = &call_anonymous_function_141};

	group_u_Field_56 *u_nothing_9 = &(group_u_Field_56){
		.u_value = u_anonymous_object_8,
		.u_name = anonymous_string_literal_7,
	};

	group_u_Text_17 *anonymous_string_literal_42 = &(group_u_Text_17){.internal_value = "parameters"};

	group_u_Field_56 *u_Function_parameters_43 = &(group_u_Field_56){
		.u_name = anonymous_string_literal_42,
		.u_value = u_anonymous_object_8,
	};

	group_u_Parameter_31 *u_anonymous_function_other_104 = &(group_u_Parameter_31){
		.u_name = anonymous_string_literal_103,
		.u_type = u_Number_90,
	};

	group_u_Group_26 *u_Anything_20 = &(group_u_Group_26){
		.u_fields = u_anonymous_list_21,
	};

	group_u_Parameter_31 *u_anonymous_function_object_125 = &(group_u_Parameter_31){
		.u_type = u_Anything_20,
		.u_name = anonymous_string_literal_124,
	};

	group_u_List_63 *u_anonymous_list_69 = &(group_u_List_63){
		.empty = '0'};

	group_u_List_63 *u_anonymous_list_147 = &(group_u_List_63){
		.empty = '0'};

	group_u_Number_90 *u_anonymous_number_142 = &(group_u_Number_90){.internal_value = 3};

	group_u_List_63 *u_anonymous_list_149 = &(group_u_List_63){
		.empty = '0'};

	group_u_Function_41 *anonymous_function_150 = &(group_u_Function_41){
		.u_compile_time_parameters = u_anonymous_list_147,
		.u_this_object = u_anonymous_number_142,
		.u_parameters = u_anonymous_list_148,
		.u_return_type = u_Number_90,
		.u_tags = u_anonymous_list_149,
		.call = &call_anonymous_function_150};

	group_u_List_63 *u_anonymous_list_25 = &(group_u_List_63){
		.empty = '0'};

	group_u_Text_17 *anonymous_string_literal_101 = &(group_u_Text_17){.internal_value = "this"};

	group_u_List_63 *u_anonymous_list_39 = &(group_u_List_63){
		.empty = '0'};

	group_u_Group_26 *u_OneOf_38 = &(group_u_Group_26){
		.u_fields = u_anonymous_list_39,
	};

	group_u_Group_26 *u_Parameter_31 = &(group_u_Group_26){
		.u_fields = u_anonymous_list_36,
	};

	group_u_BuiltinTag_79 *anonymous_object_91 = &(group_u_BuiltinTag_79){
		.u_internal_name = anonymous_string_literal_0,
	};

	group_u_List_63 *u_anonymous_list_62 = &(group_u_List_63){
		.empty = '0'};

	group_u_Field_56 *u_false_75 = &(group_u_Field_56){
		.u_value = u_anonymous_object_74,
		.u_name = anonymous_string_literal_73,
	};

	group_u_Parameter_31 *u_anonymous_function_this_102 = &(group_u_Parameter_31){
		.u_type = u_Number_90,
		.u_name = anonymous_string_literal_101,
	};

	group_u_List_63 *u_anonymous_list_121 = &(group_u_List_63){
		.empty = '0'};

	group_u_Parameter_31 *u_anonymous_function_object_137 = &(group_u_Parameter_31){
		.u_name = anonymous_string_literal_136,
		.u_type = u_Anything_20,
	};

	group_u_Parameter_31 *u_anonymous_function_other_146 = &(group_u_Parameter_31){
		.u_name = anonymous_string_literal_145,
		.u_type = u_Number_90,
	};

	group_u_List_63 *u_anonymous_list_10 = &(group_u_List_63){
		.empty = '0'};

	group_u_Group_26 *u_Either_66 = &(group_u_Group_26){
		.u_fields = u_anonymous_list_69,
	};

	group_u_Parameter_31 *u_anonymous_function_this_93 = &(group_u_Parameter_31){
		.u_name = anonymous_string_literal_92,
		.u_type = u_Number_90,
	};

	group_u_Object_23 *u_system_side_effects_54 = &(type_u_system_side_effects_54){
		.empty = '0'};

	group_u_Text_17 *anonymous_string_literal_1 = &(group_u_Text_17){.internal_value = "Number.minus"};

	group_u_Field_56 *u_Function_this_object_51 = &(group_u_Field_56){
		.u_value = u_anonymous_object_8,
		.u_name = anonymous_string_literal_50,
	};

	group_u_BuiltinTag_79 *anonymous_object_100 = &(group_u_BuiltinTag_79){
		.u_internal_name = anonymous_string_literal_1,
	};

	group_u_BuiltinTag_79 *anonymous_object_123 = &(group_u_BuiltinTag_79){
		.u_internal_name = anonymous_string_literal_2,
	};

	({
		void *arg0 = ({
			group_u_Number_90 *return_address;
			void *arg0 = u_anonymous_number_151;
			(((void (*)(group_u_Number_90 *, group_u_Number_90 *, group_u_Number_90 *))(anonymous_function_150->call))(u_anonymous_number_142, arg0, return_address));
			return_address;
		});
		(((void (*)(group_u_Anything_20 *))(anonymous_function_141->call))(arg0));
	});

	return 0;
}