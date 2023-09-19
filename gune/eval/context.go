package eval

import (
	"errors"
	"fmt"
)

type Ctx struct {
	parent    *Ctx
	variables map[string]RuntimeVal
}

func NewCtx() Ctx {
	return Ctx{variables: map[string]RuntimeVal{}}
}

func (e *Ctx) DeclareVar(identifier string, value RuntimeVal) (RuntimeVal, error) {
	if _, ok := e.variables[identifier]; ok {
		return nil, errors.New(fmt.Sprintf("%s already defined", identifier))
	}
	e.variables[identifier] = value
	return value, nil
}

func (e *Ctx) assignVar(identifier string, value RuntimeVal) (RuntimeVal, error) {
	ctx := e.resolve(identifier)
	if ctx == nil {
		return nil, errors.New(fmt.Sprintf("%s undefined", identifier))
	}
	ctx.variables[identifier] = value
	return value, nil
}

func (e *Ctx) lookupVar(identifier string) (RuntimeVal, error) {
	ctx := e.resolve(identifier)
	if ctx == nil {
		return nil, errors.New(fmt.Sprintf("%s undefined", identifier))
	}
	v, _ := ctx.variables[identifier]
	return v, nil
}

func (e *Ctx) resolve(identifier string) *Ctx {
	_, ok := e.variables[identifier]
	if ok {
		return e
	}
	if e.parent != nil {
		return e.parent.resolve(identifier)
	}
	return nil
}
