# Make an ascii art banner for the api
```
     ____                                 __        __   _____ 
    / __ \____ __________  ____ ___  ___  / /_____ _/ /  / ___/
   / / / / __ `/ ___/ __ \/ __ `__ \/ _ \/ __/ __ `/ /   \__ \ 
  / /_/ / /_/ (__  ) /_/ / / / / / /  __/ /_/ /_/ / /   ___/ / 
 /_____/\__,_/____/ .___/_/ /_/ /_/\___/\__/\__,_/_/   /____/ 
                 /_/                                          
```

This ASCII art banner can be used to promote the garage door API and draw attention to its features and benefits. It features the name of the API and a simple, stylized design that is easy to read and understand. This banner could be used on the API's website, in marketing materials, or in other contexts where it is helpful to showcase the API in a visually appealing way.

# Describe the API like a used car salesman
Hello and welcome to Garage Door Solutions, where we have a wide selection of high-quality garage door APIs to choose from! Our APIs are perfect for anyone looking to remotely control their garage doors and keep their homes safe and secure.

With our APIs, you can easily open and close your garage doors from anywhere, using just your smartphone or laptop. Plus, our advanced features allow you to automatically close your doors after a specified amount of time, so you never have to worry about leaving them open by accident.

Our APIs are easy to use and highly customizable, so you can tailor them to fit your specific needs. Whether you have a single garage door or multiple doors at different locations, we have an API that will work for you.

So why wait? Visit our website today and browse our selection of top-quality garage door APIs. Our friendly and knowledgeable sales team is standing by to assist you, so don't hesitate to contact us with any questions you may have. Trust us, with our APIs, you'll never have to worry about your garage doors again!

# API

```
# GET /api/garage/status/{doorId}
#
# Returns the current status of the specified garage door (open or closed)
#
# Path parameters:
#   doorId: the ID of the door to check
#
# Response:
# {
#   "status": "open"
# }
```

```
# POST /api/garage/open/{doorId}
#
# Opens the specified garage door and starts a timer to automatically close the door
# after the specified time has elapsed.
#
# Path parameters:
#   doorId: the ID of the door to open
#
# Query parameters:
#   timeout: the amount of time (in seconds) that the door should remain open
#
# Request body:
# {
#   "pin": "1234"
# }
#
# Response:
# {
#   "status": "success"
# }
```

```
# POST /api/garage/close/{doorId}
#
# Closes the specified garage door
#
# Path parameters:
#   doorId: the ID of the door to close
#
# Request body:
# {
#   "pin": "1234"
# }
#
# Response:
# {
#   "status": "success"
# }
```