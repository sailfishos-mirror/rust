# Test the behavior of #[link(.., kind = "raw-dylib")] and #[link_ordinal] on windows-msvc

# only-windows

include ../tools.mk

all:
	$(RUSTC) --crate-type lib --crate-name raw_dylib_test lib.rs
	$(RUSTC) --crate-type bin driver.rs -L "$(TMPDIR)"
	$(call COMPILE_OBJ,"$(TMPDIR)"/exporter.obj,exporter.c)
ifdef IS_MSVC
	$(CC) "$(TMPDIR)"/exporter.obj exporter.def -link -dll -out:"$(TMPDIR)"/exporter.dll -noimplib
else
	$(CC) "$(TMPDIR)"/exporter.obj exporter.def -shared -o "$(TMPDIR)"/exporter.dll
endif
	"$(TMPDIR)"/driver | tr -d '\r' > "$(TMPDIR)"/output.txt
	$(RUSTC_TEST_OP) "$(TMPDIR)"/output.txt output.txt
