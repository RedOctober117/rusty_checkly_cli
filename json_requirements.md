# Subscription
```json
"subscriptions": [
    {
      "id": 13968461,
      "checkId": null,
      "alertChannelId": 242455,
      "activated": true,
      "groupId": 2265339,
      "check": null,
      "checkGroup": {
        "id": 2265339,
        "name": "Group"
      }
    },
    . . . 
],
```
alert creation always contains:
```json
{
  "id": 12345,
  "type": "",
  "config": {},
  "sendRecovery": true,
  "sendFailure": true,
  "sendDegraded": true,
  "sslExpiry": false,
  "sslExpiryThreshold": 30,
  "autoSubscribe": false
}
```
# Configs

## Slack
```json
"config": {
    "url": "",
    "channel": "#testing"
},
```
## Webhook
```json
"config": {
    "name": "teams degraded test",
    "url": "",
    "method": "POST",
    "template": "",
    "headers": [],
    "queryParameters": [],
    "webhookSecret": null
},
```
## Email
```json
"config": {
    "address": "someone@somwehere.bruh"
}
```
## SMS
```json
"config": {
    "name": "whatever",
    "phoneNumber": "+2389756205"
}
```
## Call
```json
"config": {
    "phoneNumber": "+2389756205"
}
```
## Opsgenie
```json
"config": {
    "name": ""
    "region": ""
    "priority": ""
    "apiKey": ""
}
```
## Pagerduty
```json
"config": {
    "account": ""
    "serviceName": ""
    "serviceKey": ""
}
```
