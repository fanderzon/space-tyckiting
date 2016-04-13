package client

// Get valid positios where bot can move
func (b *Bot) GetValidMoves(r int) []Position {
	return getPositionsInRange(b.Position.X, b.Position.Y, r)
}

// Get valid positios where bot can use cannon
func (b *Bot) GetValidCannons(r int) []Position {
	return getPositionsInRange(0, 0, r)
}

// Get valid positios where bot can use rader
func (b *Bot) GetValidRadars(r int) []Position {
	return getPositionsInRange(0, 0, r)
}

// Get valid positions in hexagon for given radius
func getPositionsInRange(x, y, r int) (pos []Position) {
	for dx := -r; dx < r+1; dx++ {
		for dy := max(-r, -dx-r); dy < min(r, -dx+r)+1; dy++ {
			pos = append(pos, Position{dx + x, dy + y})
		}
	}
	return
}

// Select smallest integer from two integers
func min(a, b int) int {
	if b < a {
		return b
	}
	return a
}

// Select biggest integer from two integers
func max(a, b int) int {
	if b > a {
		return b
	}
	return a
}
