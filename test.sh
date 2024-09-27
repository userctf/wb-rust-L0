#!/bin/bash

test() {
SEND=$(curl 'http://127.0.0.1:3000/save_order' -s -X POST -H 'Content-Type: application/json' --data-raw $'{\n  "order_uid": "b563feb7b2b84b6test",\n  "track_number": "WBILMTESTTRACK",\n  "entry": "WBIL",\n  "delivery": {\n    "name": "Test Testov",\n    "phone": "+9720000000",\n    "zip": "2639809",\n    "city": "Kiryat Mozkin",\n    "address": "Ploshad Mira 15",\n    "region": "Kraiot",\n    "email": "test@gmail.com"\n  },\n  "payment": {\n    "transaction": "b563feb7b2b84b6test",\n    "request_id": "",\n    "currency": "USD",\n    "provider": "wbpay",\n    "amount": 1817,\n    "payment_dt": 1637907727,\n    "bank": "alpha",\n    "delivery_cost": 1500,\n    "goods_total": 317,\n    "custom_fee": 0\n  },\n  "items": [\n    {\n      "chrt_id": 9934930,\n      "track_number": "WBILMTESTTRACK",\n      "price": 453,\n      "rid": "ab4219087a764ae0btest",\n      "name": "Mascaras",\n      "sale": 30,\n      "size": "0",\n      "total_price": 317,\n      "nm_id": 2389212,\n      "brand": "Vivienne Sabo",\n      "status": 202\n    }\n  ],\n  "locale": "en",\n  "internal_signature": "",\n  "customer_id": "test",\n  "delivery_service": "meest",\n  "shardkey": "9",\n  "sm_id": 99,\n  "date_created": "2021-11-26T06:22:19Z",\n  "oof_shard": "1"\n}')

if [ "$SEND" != "{\"status\":\"success\"}" ]; then
  echo "Test failed!"
  exit 1
fi
GET=$(curl 'http://127.0.0.1:3000/get_order/b563feb7b2b84b6test' -s)
CORRECT_GET="{\"order_uid\":\"b563feb7b2b84b6test\",\"track_number\":\"WBILMTESTTRACK\",\"entry\":\"WBIL\",\"delivery\":{\"name\":\"Test Testov\",\"phone\":\"+9720000000\",\"zip\":\"2639809\",\"city\":\"Kiryat Mozkin\",\"address\":\"Ploshad Mira 15\",\"region\":\"Kraiot\",\"email\":\"test@gmail.com\"},\"payment\":{\"transaction\":\"b563feb7b2b84b6test\",\"request_id\":\"\",\"currency\":\"USD\",\"provider\":\"wbpay\",\"amount\":1817,\"payment_dt\":1637907727,\"bank\":\"alpha\",\"delivery_cost\":1500,\"goods_total\":317,\"custom_fee\":0},\"items\":[{\"chrt_id\":9934930,\"track_number\":\"WBILMTESTTRACK\",\"price\":453,\"rid\":\"ab4219087a764ae0btest\",\"name\":\"Mascaras\",\"sale\":30,\"size\":\"0\",\"total_price\":317,\"nm_id\":2389212,\"brand\":\"Vivienne Sabo\",\"status\":202}],\"locale\":\"en\",\"internal_signature\":\"\",\"customer_id\":\"test\",\"delivery_service\":\"meest\",\"shardkey\":\"9\",\"sm_id\":99,\"date_created\":\"2021-11-26T06:22:19Z\",\"oof_shard\":\"1\"}"
if [ "$GET" != "$CORRECT_GET" ]; then
  echo "Test failed!"
  exit 1
fi
}

for i in $(seq 1 100); do
    test
done

echo "Test passed"
