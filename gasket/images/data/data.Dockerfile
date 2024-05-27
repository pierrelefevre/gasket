FROM nginx 

## Copy video data
COPY data /usr/share/nginx/html

## Remove index.html
RUN rm /usr/share/nginx/html/index.html

## Nginx config (Autoindex files)
RUN rm /etc/nginx/conf.d/default.conf
COPY images/data/nginx.conf /etc/nginx/conf.d/default.conf

## Expose port 80
EXPOSE 80

## Run nginx
CMD ["nginx", "-g", "daemon off;"]
