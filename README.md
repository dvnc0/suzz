```
---------------------------------------------------------------------------
     SSSSSSSSSSSSSSS                                                     
   SS:::::::::::::::S                                                    
  S:::::SSSSSS::::::S                                                    
  S:::::S     SSSSSSS                                                    
  S:::::S            uuuuuu    uuuuuu  zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz
  S:::::S            u::::u    u::::u  z:::::::::::::::zz:::::::::::::::z
   S::::SSSS         u::::u    u::::u  z::::::::::::::z z::::::::::::::z 
    SS::::::SSSSS    u::::u    u::::u  zzzzzzzz::::::z  zzzzzzzz::::::z  
      SSS::::::::SS  u::::u    u::::u        z::::::z         z::::::z   
         SSSSSS::::S u::::u    u::::u       z::::::z         z::::::z    
              S:::::Su::::u    u::::u      z::::::z         z::::::z     
              S:::::Su:::::uuuu:::::u     z::::::z         z::::::z      
  SSSSSSS     S:::::Su:::::::::::::::uu  z::::::zzzzzzzz  z::::::zzzzzzzz
  S::::::SSSSSS:::::S u:::::::::::::::u z::::::::::::::z z::::::::::::::z
  S:::::::::::::::SS   uu::::::::uu:::uz:::::::::::::::zz:::::::::::::::z
   SSSSSSSSSSSSSSS       uuuuuuuu  uuuuzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz
---------------------------------------------------------------------------
                                                                         
A simple fuzzer that replaces the word suzz with a line from a wordlist.

Usage: suzz [OPTIONS] --file <FILE> <URL>

Arguments:
  <URL>  The URL to test

Options:
  -f, --file <FILE>    Word list to use
  -d, --delay <DELAY>  Optional delay in seconds between requests [default: 0]
  -v, --verbose        Emable verbose output
  -H, --head           HEAD request instead of GET
  -h, --help           Print help
  -V, --version        Print version
```