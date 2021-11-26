FROM node:16

WORKDIR /usr/src/app

COPY package.json /usr/src/app/
RUN yarn

RUN echo "HELLO WORLD" > /tmp/lmao

ENV PATH="/usr/src/app/node_modules/.bin:$PATH"
CMD ["ng", "serve", "--host", "0.0.0.0"]