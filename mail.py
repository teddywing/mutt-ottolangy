#!/usr/bin/env python3

# from email import parser
import email
import sys

# p = parser.BytesFeedParser()
# p.feed(sys.stdin)
# msg = p.close()

with open('en-mail', 'r') as f:
    msg = email.message_from_file(f)

    # print(msg.items())
    # print(msg.get_body())
    # print(type(msg))
    # for p in msg.iter_parts():
    #     print(p)

    for p in msg.walk():
        print(p.get_content_type())

        if p.get_content_type() == 'text/plain':
            print(dir(p))
            print(p.get_payload())
