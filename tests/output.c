#include <stdio.h>
#include <stdlib.h>

typedef struct u_This u_This;
typedef struct u_List u_List;
typedef struct u_Object u_Object;
typedef struct u_Error u_Error;
typedef struct u_Text u_Text;
typedef struct u_Field u_Field;
typedef struct u_Either u_Either;
typedef struct u_Anything u_Anything;
typedef struct u_Number u_Number;
typedef struct u_OneOf u_OneOf;
typedef struct u_Parameter u_Parameter;
typedef struct u_BuiltinTag u_BuiltinTag;
typedef struct u_Group u_Group;

struct u_This {
};

struct u_List {
};

struct u_Object {
};

struct u_Error {
};

struct u_Text {
};

u_Anything anonymous_function_1(u_Anything* u_this, u_Anything* u_other, void* return_address) {
	printf("%s", u_this);
}

struct u_Field {
};

struct u_Either {
};

void anonymous_function_3(u_Anything* u_object, void* return_address) {
	printf("%s", u_object);
}

struct u_Anything {
};

void u_builtin(void* return_address) {
	({
	goto label
	})
}

u_Text anonymous_function_4(void* return_address) {
	char* buffer;
	size_t length;
	getline(&buffer, &size, stdin);
	*return_address = buffer;
}

void anonymous_function_3(u_Anything* u_object, void* return_address) {
	printf("%s", u_object);
}

struct u_Number {
	void* u_plus;
	void* u_minus;
};

struct u_OneOf {
};

struct u_Parameter {
};

u_Anything anonymous_function_2(u_Anything* u_this, u_Anything* u_other, void* return_address) {
	printf("%s", u_this);
}

struct u_BuiltinTag {
};

struct u_Group {
};

int main(int argc, char** argv) {
	void* POINTER_98 = &(u_List) {};
	
	void* POINTER_87 = &(u_List) {};
	
	void* POINTER_91 = &(u_List) {};
	
	void* POINTER_68 = &(u_List) {};
	
	void* anonymous_string_literal_2 = &(u_Text) { .internal_value = "terminal.print" };
	
	void* POINTER_22 = &(u_List) {};
	
	void* POINTER_67 = &(u_List) {};
	
	void* POINTER_15 = &(u_OneOf) {
		.u_variants = POINTER_12,
		.u_compile_time_parameters = POINTER_14,
	};
	
	void* anonymous_string_literal_14 = &(u_Text) { .internal_value = "plus" };
	
	void* anonymous_string_literal_12 = &(u_Text) { .internal_value = "this" };
	
	void* POINTER_44 = &(u_List) {};
	
	void* POINTER_99 = &(u_List) {};
	
	void* POINTER_18 = &(u_List) {};
	
	void* POINTER_38 = &(u_Field) {
		.u_name = anonymous_string_literal_7,
		.u_value = POINTER_37,
	};
	
	void* POINTER_42 = &(u_List) {};
	
	void* POINTER_49 = &(u_List) {};
	
	void* anonymous_string_literal_6 = &(u_Text) { .internal_value = "Data" };
	
	void* u_anonymous_function_1_this = &(u_Parameter) {
		.u_type = u_Anything,
		.u_name = anonymous_string_literal_10,
	};
	
	void* POINTER_30 = &(u_List) {};
	
	void* u_anonymous_function_2_other = &(u_Parameter) {
		.u_name = anonymous_string_literal_13,
		.u_type = u_Anything,
	};
	
	void* POINTER_97 = &(u_List) {};
	
	void* POINTER_9 = &(u_Field) {
		.u_name = anonymous_string_literal_5,
		.u_value = POINTER_8,
	};
	
	void* POINTER_37 = &(u_Object) {};
	
	void* POINTER_76 = &(u_List) {};
	
	void* anonymous_string_literal_19 = &(u_Text) { .internal_value = "Hello world!" };
	
	void* POINTER_85 = &(u_List) {};
	
	void* u_Number_minus = &(u_Field) {
		.u_name = anonymous_string_literal_15,
		.u_value = anonymous_function_2,
	};
	
	void* POINTER_40 = &(u_Object) {};
	
	void* anonymous_string_literal_13 = &(u_Text) { .internal_value = "other" };
	
	void* anonymous_string_literal_4 = &(u_Text) { .internal_value = "Hello world!" };
	
	void* POINTER_5 = &(u_List) {};
	
	void* POINTER_43 = &(u_Either) {
		.u_variants = POINTER_42,
	};
	
	void* u_anonymous_function_1_other = &(u_Parameter) {
		.u_type = u_Anything,
		.u_name = anonymous_string_literal_11,
	};
	
	void* u_builtin_name = &(u_Parameter) {
		.u_name = anonymous_string_literal_9,
		.u_type = u_Text,
	};
	
	void* POINTER_24 = &(u_List) {};
	
	void* anonymous_string_literal_7 = &(u_Text) { .internal_value = "true" };
	
	void* anonymous_string_literal_9 = &(u_Text) { .internal_value = "name" };
	
	void* anonymous_string_literal_15 = &(u_Text) { .internal_value = "minus" };
	
	void* anonymous_object_2 = &(u_BuiltinTag) {
		.u_internal_name = anonymous_string_literal_2,
	};
	
	void* POINTER_80 = &(u_List) {};
	
	void* u_system_side_effects = &(u_Object) {};
	
	void* anonymous_string_literal_17 = &(u_Text) { .internal_value = "object" };
	
	void* POINTER_92 = &(u_List) {};
	
	void* POINTER_90 = &(u_List) {};
	
	void* anonymous_string_literal_1 = &(u_Text) { .internal_value = "Number.minus" };
	
	void* u_anonymous_function_2_this = &(u_Parameter) {
		.u_type = u_Anything,
		.u_name = anonymous_string_literal_12,
	};
	
	void* u_cabin_only = &(u_Object) {};
	
	void* POINTER_32 = &(u_List) {};
	
	void* POINTER_12 = &(u_List) {};
	
	void* anonymous_object_2 = &(u_BuiltinTag) {
		.u_internal_name = anonymous_string_literal_0,
	};
	
	void* anonymous_string_literal_5 = &(u_Text) { .internal_value = "nothing" };
	
	void* POINTER_26 = &(u_List) {};
	
	void* POINTER_57 = &(u_List) {};
	
	void* POINTER_59 = &(u_List) {};
	
	void* u_Number_plus = &(u_Field) {
		.u_value = anonymous_function_1,
		.u_name = anonymous_string_literal_14,
	};
	
	void* POINTER_20 = &(u_List) {};
	
	void* anonymous_string_literal_16 = &(u_Text) { .internal_value = "Data" };
	
	void* anonymous_string_literal_18 = &(u_Text) { .internal_value = "object" };
	
	void* POINTER_14 = &(u_List) {};
	
	void* u_anonymous_function_3_object = &(u_Parameter) {
		.u_name = anonymous_string_literal_18,
		.u_type = u_Anything,
	};
	
	void* POINTER_8 = &(u_Object) {};
	
	void* u_anonymous_function_3_object = &(u_Parameter) {
		.u_type = u_Anything,
		.u_name = anonymous_string_literal_17,
	};
	
	void* POINTER_81 = &(u_OneOf) {
		.u_compile_time_parameters = POINTER_80,
		.u_variants = POINTER_78,
	};
	
	void* anonymous_object_2 = &(u_BuiltinTag) {
		.u_internal_name = anonymous_string_literal_3,
	};
	
	void* u_terminal = &(u_Object) {
		.u_input = anonymous_function_4,
		.u_print = anonymous_function_3,
	};
	
	void* anonymous_string_literal_11 = &(u_Text) { .internal_value = "other" };
	
	void* POINTER_58 = &(u_List) {};
	
	void* POINTER_48 = &(u_List) {};
	
	void* anonymous_string_literal_8 = &(u_Text) { .internal_value = "false" };
	
	void* POINTER_66 = &(u_List) {};
	
	void* anonymous_string_literal_0 = &(u_Text) { .internal_value = "Number.plus" };
	
	void* POINTER_78 = &(u_List) {};
	
	void* POINTER_34 = &(u_List) {};
	
	void* POINTER_86 = &(u_List) {};
	
	void* anonymous_string_literal_10 = &(u_Text) { .internal_value = "this" };
	
	void* anonymous_string_literal_3 = &(u_Text) { .internal_value = "terminal.input" };
	
	void* POINTER_11 = &(u_Either) {
		.u_variants = POINTER_10,
	};
	
	void* POINTER_50 = &(u_List) {};
	
	void* POINTER_16 = &(u_List) {};
	
	void* anonymous_object_2 = &(u_BuiltinTag) {
		.u_internal_name = anonymous_string_literal_1,
	};
	
	void* POINTER_74 = &(u_List) {};
	
	void* POINTER_10 = &(u_List) {};
	
	void* POINTER_41 = &(u_Field) {
		.u_value = POINTER_40,
		.u_name = anonymous_string_literal_8,
	};
	
	void* u_This = &u_This;
	void* u_Nothing = &POINTER_11;
	void* u_nothing = &POINTER_8;
	void* u_Optional = &POINTER_15;
	void* u_Text = &u_Text;
	void* u_Anything = &u_Anything;
	void* u_Object = &u_Object;
	void* u_Group = &u_Group;
	void* u_Parameter = &u_Parameter;
	void* u_OneOf = &u_OneOf;
	void* u_cabin_only = &u_cabin_only;
	void* u_system_side_effects = &u_system_side_effects;
	void* u_Field = &u_Field;
	void* u_List = &u_List;
	void* u_Either = &u_Either;
	void* u_Boolean = &POINTER_43;
	void* u_true = &POINTER_37;
	void* u_false = &POINTER_40;
	void* u_BuiltinTag = &u_BuiltinTag;
	void* u_builtin = &u_builtin;
	void* u_Number = &u_Number;
	void* u_Error = &u_Error;
	void* u_Result = &POINTER_81;
	void* u_terminal = &u_terminal;
	void;

	return 0;
}