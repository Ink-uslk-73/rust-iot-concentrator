# certs/ - сертификаты для mTLS

## 1. CA
openssl req -x509 -newkey rsa:4096 -days 3650 -nodes -keyout ca.key -out ca.crt -subj "/CN=Concentrator-CA/O=IOT"

## 2. Сервер
openssl req -newkey rsa:2048 -nodes -keyout server.key -out server.csr -subj "/CN=concentrator.local/O=IOT"
openssl x509 -req -in server.csr -CA ca.crt -CAkey ca.key -CAcreateserial -out server.crt -days 825

## 3. Планшеты
openssl req -newkey rsa:2048 -nodes -keyout tablet-01.key -out tablet-01.csr -subj "/CN=tablet-01/O=IOT"
openssl x509 -req -in tablet-01.csr -CA ca.crt -CAkey ca.key -CAcreateserial -out tablet-01.crt -days 825
