'use strict';

const app    = require('express')();
const routes = require('./routes');

app.set('view engine', 'jade');
app.set('views', './app/views');

app.use('/', routes);

const FAQ = require('./models/faq');
FAQ.create({ question: 'foo', answer: 'bar' });

module.exports = app;
