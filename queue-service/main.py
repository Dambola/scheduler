from queue_service.app import create_app

app = create_app()
app.run()

# from time import sleep
# from queue_service.utils import timed

# with timed('FuncTimed'):
#     sleep(1)