from python_sdk import rust_sleep
import asyncio

async def main():
    print('starting')
    result = await rust_sleep()
    print(f'result {result.origin}')

asyncio.run(main())