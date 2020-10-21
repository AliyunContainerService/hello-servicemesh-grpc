package main

import (
	"fmt"
	log "github.com/sirupsen/logrus"
	"path/filepath"
	"runtime"
	"strings"
)

func Infof(format string, args ...interface{}) {
	filename, line, funcname, funcname1, funcname2 := buildLogParams()
	log.Infof("%s[%d]:%s<-%s<-%s: %s\n", filename, line, funcname, funcname1, funcname2, fmt.Sprintf(format, args...))
}

func buildLogParams() (string, int, string, string, string) {
	return buildLogParams0(3)
}

func buildLogParams0(skip int) (string, int, string, string, string) {
	funcName, funcName1, funcname2, line, filename := "???", "???", "???", 0, "???"
	pc, filename, line, ok := runtime.Caller(skip)
	if ok {
		funcName = runtime.FuncForPC(pc).Name()
		funcName = filepath.Ext(funcName)
		funcName = strings.TrimPrefix(funcName, ".")
		filename = filepath.Base(filename)
	}
	pc1, _, _, ok := runtime.Caller(skip + 1)
	if ok {
		funcName1 = runtime.FuncForPC(pc1).Name()
		funcName1 = filepath.Ext(funcName1)
		funcName1 = strings.TrimPrefix(funcName1, ".")
	}
	pc2, _, _, ok := runtime.Caller(skip + 2)
	if ok {
		funcname2 = runtime.FuncForPC(pc2).Name()
		funcname2 = filepath.Ext(funcname2)
		funcname2 = strings.TrimPrefix(funcname2, ".")
	}
	return filename, line, funcName, funcName1, funcname2
}

/*
func main() {
	test()
}

func test() {
	test2()
}

func test2() {
	test3()
}
func test3() {
	Infof("hello %s", "eric")
}
*/