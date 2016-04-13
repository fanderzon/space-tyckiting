package client

import (
	"testing"
)

type TestAi struct {
}

func TestClient(t *testing.T) {
	a := &TestAi{}
	Run(a, "Test")
}

func (t *TestAi) Move() (actions []Action) {
	return actions
}

func (t *TestAi) OnConnected(msg ConnectedMessage) {
}

func (t *TestAi) OnStart(msg StartMessage) {
}

func (t *TestAi) OnEvents(msg EventsMessage) {
}

func (t *TestAi) OnEnd(msg EndMessage) {
}

func (t *TestAi) OnError(msg string) {
}
