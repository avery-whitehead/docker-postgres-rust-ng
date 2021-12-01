FROM node:16

WORKDIR /usr/web
COPY . /usr/web/
RUN yarn install
ENV PATH=/usr/web/node_modules/.bin:$PATH
EXPOSE 4200

CMD ["ng", "serve", "--host", "0.0.0.0"]