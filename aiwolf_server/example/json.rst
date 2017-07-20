Serverによる通信の例
====================

Server側のする、JSON形式でのデータの例。

.. code-block:: javascript
    :linenos:

    {
      "gameInfo": {
        "agent": 10,
        "attackVoteList": [],
        "attackedAgent": -1,
        "cursedFox": -1,
        "day": 5,
        "divineResult": null,
        "executedAgent": 1,
        "existingRoleList": [
          "BODYGUARD",
          "MEDIUM",
          "POSSESSED",
          "SEER",
          "VILLAGER",
          "WEREWOLF"
        ],
        "guardedAgent": -1,
        "lastDeadAgentList": [],
        "latestAttackVoteList": [],
        "latestExecutedAgent": -1,
        "latestVoteList": [
          {
            "agent": 3,
            "day": 5,
            "target": 6
          },
          {
            "agent": 4,
            "day": 5,
            "target": 5
          },
          {
            "agent": 5,
            "day": 5,
            "target": 8
          },
          {
            "agent": 6,
            "day": 5,
            "target": 10
          },
          {
            "agent": 8,
            "day": 5,
            "target": 13
          },
          {
            "agent": 9,
            "day": 5,
            "target": 15
          },
          {
            "agent": 10,
            "day": 5,
            "target": 6
          },
          {
            "agent": 13,
            "day": 5,
            "target": 5
          },
          {
            "agent": 14,
            "day": 5,
            "target": 8
          },
          {
            "agent": 15,
            "day": 5,
            "target": 3
          }
        ],
        "mediumResult": null,
        "remainTalkMap": {
          "3": 9,
          "4": 9,
          "5": 8,
          "6": 9,
          "8": 9,
          "9": 9,
          "10": 9,
          "13": 9,
          "14": 9,
          "15": 9
        },
        "remainWhisperMap": {},
        "roleMap": {
          "10": "VILLAGER"
        },
        "statusMap": {
          "1": "DEAD",
          "2": "DEAD",
          "3": "ALIVE",
          "4": "ALIVE",
          "5": "ALIVE",
          "6": "ALIVE",
          "7": "DEAD",
          "8": "ALIVE",
          "9": "ALIVE",
          "10": "ALIVE",
          "11": "DEAD",
          "12": "DEAD",
          "13": "ALIVE",
          "14": "ALIVE",
          "15": "ALIVE"
        },
        "talkList": [
          {
            "agent": 15,
            "day": 5,
            "idx": 0,
            "text": "VOTE Agent[03]",
            "turn": 0
          },
          {
            "agent": 3,
            "day": 5,
            "idx": 1,
            "text": "VOTE Agent[06]",
            "turn": 0
          },
          {
            "agent": 14,
            "day": 5,
            "idx": 2,
            "text": "VOTE Agent[08]",
            "turn": 0
          },
          {
            "agent": 9,
            "day": 5,
            "idx": 3,
            "text": "VOTE Agent[15]",
            "turn": 0
          },
          {
            "agent": 6,
            "day": 5,
            "idx": 4,
            "text": "VOTE Agent[10]",
            "turn": 0
          },
          {
            "agent": 13,
            "day": 5,
            "idx": 5,
            "text": "VOTE Agent[05]",
            "turn": 0
          },
          {
            "agent": 10,
            "day": 5,
            "idx": 6,
            "text": "VOTE Agent[06]",
            "turn": 0
          },
          {
            "agent": 8,
            "day": 5,
            "idx": 7,
            "text": "VOTE Agent[13]",
            "turn": 0
          },
          {
            "agent": 5,
            "day": 5,
            "idx": 8,
            "text": "VOTE Agent[08]",
            "turn": 0
          },
          {
            "agent": 4,
            "day": 5,
            "idx": 9,
            "text": "VOTE Agent[05]",
            "turn": 0
          },
          {
            "agent": 5,
            "day": 5,
            "idx": 10,
            "text": "REQUEST(VOTE Agent[08])",
            "turn": 1
          },
          {
            "agent": 14,
            "day": 5,
            "idx": 11,
            "text": "Skip",
            "turn": 1
          },
          {
            "agent": 13,
            "day": 5,
            "idx": 12,
            "text": "Skip",
            "turn": 1
          },
          {
            "agent": 4,
            "day": 5,
            "idx": 13,
            "text": "Skip",
            "turn": 1
          },
          {
            "agent": 10,
            "day": 5,
            "idx": 14,
            "text": "Skip",
            "turn": 1
          },
          {
            "agent": 6,
            "day": 5,
            "idx": 15,
            "text": "Skip",
            "turn": 1
          },
          {
            "agent": 8,
            "day": 5,
            "idx": 16,
            "text": "Skip",
            "turn": 1
          },
          {
            "agent": 3,
            "day": 5,
            "idx": 17,
            "text": "Skip",
            "turn": 1
          },
          {
            "agent": 9,
            "day": 5,
            "idx": 18,
            "text": "Skip",
            "turn": 1
          },
          {
            "agent": 15,
            "day": 5,
            "idx": 19,
            "text": "Skip",
            "turn": 1
          },
          {
            "agent": 9,
            "day": 5,
            "idx": 20,
            "text": "Skip",
            "turn": 2
          },
          {
            "agent": 14,
            "day": 5,
            "idx": 21,
            "text": "Skip",
            "turn": 2
          },
          {
            "agent": 3,
            "day": 5,
            "idx": 22,
            "text": "Skip",
            "turn": 2
          },
          {
            "agent": 15,
            "day": 5,
            "idx": 23,
            "text": "Skip",
            "turn": 2
          },
          {
            "agent": 5,
            "day": 5,
            "idx": 24,
            "text": "Skip",
            "turn": 2
          },
          {
            "agent": 4,
            "day": 5,
            "idx": 25,
            "text": "Skip",
            "turn": 2
          },
          {
            "agent": 8,
            "day": 5,
            "idx": 26,
            "text": "Skip",
            "turn": 2
          },
          {
            "agent": 13,
            "day": 5,
            "idx": 27,
            "text": "Skip",
            "turn": 2
          },
          {
            "agent": 10,
            "day": 5,
            "idx": 28,
            "text": "Skip",
            "turn": 2
          },
          {
            "agent": 6,
            "day": 5,
            "idx": 29,
            "text": "Skip",
            "turn": 2
          },
          {
            "agent": 8,
            "day": 5,
            "idx": 30,
            "text": "Over",
            "turn": 3
          },
          {
            "agent": 5,
            "day": 5,
            "idx": 31,
            "text": "Skip",
            "turn": 3
          },
          {
            "agent": 15,
            "day": 5,
            "idx": 32,
            "text": "Over",
            "turn": 3
          },
          {
            "agent": 10,
            "day": 5,
            "idx": 33,
            "text": "Over",
            "turn": 3
          },
          {
            "agent": 9,
            "day": 5,
            "idx": 34,
            "text": "Over",
            "turn": 3
          },
          {
            "agent": 13,
            "day": 5,
            "idx": 35,
            "text": "Over",
            "turn": 3
          },
          {
            "agent": 4,
            "day": 5,
            "idx": 36,
            "text": "Over",
            "turn": 3
          },
          {
            "agent": 3,
            "day": 5,
            "idx": 37,
            "text": "Over",
            "turn": 3
          },
          {
            "agent": 6,
            "day": 5,
            "idx": 38,
            "text": "Over",
            "turn": 3
          },
          {
            "agent": 14,
            "day": 5,
            "idx": 39,
            "text": "Over",
            "turn": 3
          },
          {
            "agent": 8,
            "day": 5,
            "idx": 40,
            "text": "Over",
            "turn": 4
          },
          {
            "agent": 5,
            "day": 5,
            "idx": 41,
            "text": "Over",
            "turn": 4
          },
          {
            "agent": 4,
            "day": 5,
            "idx": 42,
            "text": "Over",
            "turn": 4
          },
          {
            "agent": 9,
            "day": 5,
            "idx": 43,
            "text": "Over",
            "turn": 4
          },
          {
            "agent": 13,
            "day": 5,
            "idx": 44,
            "text": "Over",
            "turn": 4
          },
          {
            "agent": 10,
            "day": 5,
            "idx": 45,
            "text": "Over",
            "turn": 4
          },
          {
            "agent": 15,
            "day": 5,
            "idx": 46,
            "text": "Over",
            "turn": 4
          },
          {
            "agent": 6,
            "day": 5,
            "idx": 47,
            "text": "Over",
            "turn": 4
          },
          {
            "agent": 3,
            "day": 5,
            "idx": 48,
            "text": "Over",
            "turn": 4
          },
          {
            "agent": 14,
            "day": 5,
            "idx": 49,
            "text": "Over",
            "turn": 4
          }
        ],
        "voteList": [
          {
            "agent": 1,
            "day": 4,
            "target": 8
          },
          {
            "agent": 3,
            "day": 4,
            "target": 1
          },
          {
            "agent": 4,
            "day": 4,
            "target": 1
          },
          {
            "agent": 5,
            "day": 4,
            "target": 6
          },
          {
            "agent": 6,
            "day": 4,
            "target": 1
          },
          {
            "agent": 8,
            "day": 4,
            "target": 1
          },
          {
            "agent": 9,
            "day": 4,
            "target": 1
          },
          {
            "agent": 10,
            "day": 4,
            "target": 1
          },
          {
            "agent": 13,
            "day": 4,
            "target": 1
          },
          {
            "agent": 14,
            "day": 4,
            "target": 1
          },
          {
            "agent": 15,
            "day": 4,
            "target": 1
          }
        ],
        "whisperList": []
      },
      "gameSetting": null,
      "request": "VOTE",
      "talkHistory": null,
      "whisperHistory": null
    }