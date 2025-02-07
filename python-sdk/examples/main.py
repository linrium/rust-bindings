from python_sdk import rust_sleep
import asyncio

async def main():
    print('starting')
    try:
        result = await rust_sleep()
        print(f'result {result.origin}')
    except Exception as e:
        print(e)

asyncio.run(main())