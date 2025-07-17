# use an official python runtime as a parent image
FROM python:3.11-slim

# set the working directory in the container
WORKDIR /usr/src/app
# copy the current directory into the container at /usr/src/app
COPY . .

# install any needed packages specified in requirements.txt
RUN pip install --no-cache-dir -r requirements.txt

# make port 80 available to the world outside this container
EXPOSE 80