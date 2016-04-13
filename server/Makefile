.PHONY : all eslint jscs

SRC=start-server.js spectator/*.js server/ test/

BINDIR=node_modules/.bin
ESLINT=$(BINDIR)/eslint
JSCS=$(BINDIR)/jscs
MOCHA=$(BINDIR)/mocha
IMOCHA=$(BINDIR)/_mocha
ISTANBUL=$(BINDIR)/istanbul

all : jscs eslint mocha istanbul

eslint :
	$(ESLINT) $(SRC)

jscs :
	$(JSCS) $(SRC)

mocha :
	$(MOCHA) test

istanbul :
	$(ISTANBUL) cover $(IMOCHA) test
	test -f coverage/coverage.json
	$(ISTANBUL) check-coverage --statements 100 --branches 100 --functions 100 coverage/coverage.json
