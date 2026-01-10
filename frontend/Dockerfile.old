# https://www.docker.com/blog/how-to-dockerize-react-app/
FROM node:23-alpine3.20 as build

WORKDIR /app
ARG BACKEND_PORT
ENV VITE_BASE_URL=http://localhost:${BACKEND_PORT}/api/v2

COPY package*.json .
RUN npm install
COPY . .
# RUN npm run build

# FROM nginx:stable-alpine as production
# COPY --from=build /app/dist /usr/share/nginx/html
# COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 3000
# CMD [ "nginx", "-g", "daemon off;" ]
CMD [ "npm", "run", "dev", "--", "--host" ]
