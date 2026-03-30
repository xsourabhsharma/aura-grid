import requests
import json
import time

CONTROLLER_URL = "http://localhost:3000/scan"

def test_vulnerable_contract():
    print("🚀 Sending test contract to Aura-Grid...")
    
    payload = {
        "contract_id": "0xTEST_VULN_REENTRANCY",
        "code": "contract Vulnerable { function withdraw() public { msg.sender.call{value: 1 ether}(''); balances[msg.sender] = 0; } }"
    }
    
    try:
        response = requests.post(CONTROLLER_URL, json=payload)
        if response.status_code == 200:
            print(f"✅ Task Queued: {response.json()}")
        else:
            print(f"❌ Error: {response.status_code} - {response.text}")
    except Exception as e:
        print(f"❌ Failed to connect to Controller: {e}")

if __name__ == "__main__":
    test_vulnerable_contract()
