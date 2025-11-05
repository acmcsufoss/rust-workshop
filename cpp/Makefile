CXX ?= clang++
builddir := build/
pname := program
target := $(builddir)$(pname)
srcs := $(wildcard *.cpp)
headers := $(wildcard *.hpp)

all: $(target)

run: $(target)
	./$(target)

$(target): $(srcs) $(headers)
	@mkdir -p $(builddir)
	$(CXX) $(srcs) -o $@

.PHONY: all run
