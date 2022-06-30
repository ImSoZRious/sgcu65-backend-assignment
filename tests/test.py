from helper import get, post
from user_test import user_test
from task_test import task_test
from assign_test import assign_test

def main():
  print('[USER TEST]')
  user_test()

  print('[TASK TEST]')
  task_test()

  print('[ASSIGN TEST]')
  assign_test()

if __name__ == '__main__':
  main()