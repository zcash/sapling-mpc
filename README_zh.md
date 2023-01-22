# 多方计算

多方计算(Multi-party Computation，MPC)是创建匿名交易需要的公共参数的的第二步，本文档说明如何参与TRONZ组织的mpc以及如何验证最终公共参数。

## 如何参与 ? 

1. 请发送邮件到tronz_mpc@tronz.io申请参与mpc，请简要介绍自己，并说明合适的参与时间。

2. TRONZ会回复邮件告知你参与mpc的时间点。

3. 当轮到你参与的时候，TRONZ给你发送一封通知邮件，包括你的序号和`params`文件的链接。
4. 下载并安装IM工具 [keybase](https://keybase.io/)，如果你没有账户新建一个。有任何问题，请添加`tronz_mpc`为好友来解决。
5. 准备好你的服务器，推荐的最小硬件要求如下：
   + Processor (CPU): Intel Core i5 (第六代或以上) or equivalent
   + Operating System: Linux (推荐) / macOS / Windows 7+         
   + Memory: 8 GB RAM
   + Storage: 500 GB internal storage drive

&emsp;&emsp;如果你使用Windows系统，参考[rust](https://www.rust-lang.org/learn/get-started) 官方主页来安装。

&emsp;&emsp;如果你使用macOS系统, 请先安装xcode命令行工具：
```
# xcode-select --install
```
6. 下载mpc项目的源码：
```
# git clone https://github.com/tronprotocol/mpc
```
&emsp;&emsp;如果你还没有安装git环境，先安装 [git](https://git-scm.com/downloads).

7. 如果你的系统是macOS/Linux，打开终端切换到mpc目录下，运行以下命令，如有提示请选择1.
```
# sh compute.sh <your_params_link>
```
如果你的系统是Windows, 下载`params`文件放在mpc目录下，然后在该目录下执行以下命令：
```
# cargo run --release --bin compute
```
&emsp;&emsp;当计算完成后，程序输出一个`new_params`文件和hash，请保存该hash值。类似于以下：
![](https://raw.githubusercontent.com/tronprotocol/documentation-en/master/docs_without_index/internal-test/mpc-output.jpg)

&emsp;&emsp;这个计算过程大约耗时半小时~1小时，根据硬件环境可能稍有不同，占用1.5 ~ 2GB物理内存。

8. 请把你的结果告知TRONZ。通过Keybase来发送`new_params`文件，然后把hash值通过邮件告知我们，告知操作系统版本更好，例如Windows 7/Centos 7。

## 依赖
我们重用了zcash的多方计算第二阶段的最终结果 https://download.z.cash/sapling-mpc/params 作为我们的起始参数，添加额外一些参与者。每个参与者把结果发送给我们后，我们会验证生成的参数是否有效。在最后一个参与者结束迭代计算后，我们采用比特币最新区块的hash值作为伪随机函数的种子来生成随机信标（Random Beacon）。在mpc结束后，我们把所有参与者的贡献发布在本项目的[wiki](https://github.com/tronprotocol/mpc/wiki) 。所有参与者可以验证自己的贡献是否保存在最终参数中。


## 如何验证我的贡献 ?

在所有参与者完成迭代后，我们把最终的`params`发布于aws上，永久存储。可以通过以下方式验证：
```
# cd mpc
# wget https://mpcfilepub.s3.amazonaws.com/mpc/params
# wget https://mpcfilepub.s3.amazonaws.com/powersoftau/powersoftau_phase1_files.tar
# tar xvf powersoftau_phase1_files.tar && cp powersoftau_phase1_files/* .
# cargo run --release --features="verification" --bin verify
```
验证过程大约耗时一小时，输出所有参与者的hash值。查询自己保存的hash值是否在输出的hash列表中。

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
