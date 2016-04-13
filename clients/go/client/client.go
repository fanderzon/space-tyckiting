// Client package provides connection to space tyckiting server
package client

import (
	"encoding/json"
	"flag"
	"fmt"

	"github.com/gorilla/websocket"
)

type Client struct {
	ai      Ai
	conn    *websocket.Conn
	addr    string
	verbose bool
}

// Ai is an interface that custom ai has to implement
type Ai interface {
	Move() []Action
	OnStart(msg StartMessage)
	OnConnected(msg ConnectedMessage)
	OnEvents(msg EventsMessage)
	OnEnd(msg EndMessage)
	OnError(msg string)
}

var joinMsg = JoinMessage{
	Type:     CLIENT_JOIN,
	TeamName: "",
}

// Run function creates client for Ai, opens websocket and starts reading from it.
func Run(ai Ai, name string) (err error) {
	c := Client{ai: ai}
	c.parseArgs()

	// If connection fails return error
	//var res *http.Response
	if c.conn, _, err = websocket.DefaultDialer.Dial(c.addr, nil); err != nil {
		c.Log(fmt.Sprintf("[client][error] : Could not connec to %s, %s ", c.addr, err))
		return
	}

	// Add ai's name in join message
	joinMsg.TeamName = name

	c.Log(fmt.Sprintf("[client][verbose] : Connected to %s", c.addr))
	defer c.conn.Close()

	// Start reading websocket
	c.read()
	return
}

// parseArgs parses host, port and verbose flag from command line arguments.
func (c *Client) parseArgs() {
	host := flag.String("h", "localhost", "Host to connect to")
	port := flag.Uint("p", 3000, "Host port")
	verbose := flag.Bool("v", false, "Verbose output")

	flag.Parse()
	c.addr = fmt.Sprintf("ws://%s:%d", *host, *port)
	c.verbose = *verbose
}

// read function reads data from websocket as long as it is open.
func (c *Client) read() {
	var data []byte
	var err error
	var msg ServerMessage
	actionsMsg := ActionsMessage{Type: CLIENT_ACTIONS}

	for {
		// Read message from socket
		_, data, err = c.conn.ReadMessage()
		if err != nil {
			c.Log(fmt.Sprintf("[client][websocket] : Disconnected %s\n", err))
			return
		}
		c.Log(fmt.Sprintf("[client][RX] : %s", data))

		// Parse message type
		if err = json.Unmarshal(data, &msg); err != nil {
			c.Log(fmt.Sprintf("[client][error] : Could not parse message type from %s, %s", data, err))
			continue
		}

		// Parse message type and call ai's function based on that type
		switch msg.Type {

		case SERVER_CONNECTED:
			var connMsg ConnectedMessage
			if err = json.Unmarshal(data, &connMsg); err != nil {
				c.Log(fmt.Sprintf("[client][error] : Could not parse connected message %s, %s", data, err))
				continue
			}
			c.ai.OnConnected(connMsg)

			// Send join message after successful connection
			if c.conn.WriteJSON(joinMsg); err != nil {
				c.Log(fmt.Sprintf("[client][error] : Could not send join message %s", err))
			}
			break

		case SERVER_START:
			var startMsg StartMessage
			if err = json.Unmarshal(data, &startMsg); err != nil {
				c.Log(fmt.Sprintf("[client][error] : Could not parse start message %s, %s", data, err))
				continue
			}
			c.ai.OnStart(startMsg)
			break

		case SERVER_EVENTS:
			var eventsMsg EventsMessage
			if err = json.Unmarshal(data, &eventsMsg); err != nil {
				c.Log(fmt.Sprintf("[client][error] : Could not parse events message %s, %s", data, err))
				continue
			}
			c.ai.OnEvents(eventsMsg)

			// Get moves from ai and send to server
			actionsMsg.RoundId = eventsMsg.RoundId
			actionsMsg.Actions = c.ai.Move()

			if c.conn.WriteJSON(actionsMsg); err != nil {
				c.Log(fmt.Sprintf("[client][error] : Could not send join message %s", err))
			}

			break

		case SERVER_END:
			var endMsg EndMessage
			if err = json.Unmarshal(data, &endMsg); err != nil {
				c.Log(fmt.Sprintf("[client][error] : Could not parse end message %s, %s", data, err))
				continue
			}
			c.ai.OnEnd(endMsg)
			break

		default:
			c.ai.OnError(string(data))
			break

		}
	}
}

// Log funtion prints string to stdout if verbose is set to true
func (c *Client) Log(s string) {
	if c.verbose {
		fmt.Println(s)
	}
}
