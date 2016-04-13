.PHONY : all eslint

SRC=cli.js main.js ai/ test/

ESLINT=node_modules/.bin/eslint
MOCHA=node_modules/.bin/mocha

all : eslint mocha

eslint :
	$(ESLINT) $(SRC)

mocha :
	$(MOCHA) test