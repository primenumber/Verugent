﻿#![allow(dead_code)]
#![allow(non_snake_case)]
use std::ops::*;
use std::string::String;
use std::io::Write;
use std::fs::OpenOptions;

use std::*;

/** 
  * 構文生成マクロ
  * オーバーロードで対応できない構文用のマクロ
  **/
#[macro_export]
macro_rules! F {
    ($lhs:ident == $rhs:expr) => {
        ($lhs.clone()).eq($rhs.clone())
    };

    ($lhs:ident != $rhs:expr) => {
        ($lhs.clone()).ne($rhs.clone())
    };

    ($lhs:ident <= $rhs:expr) => {
        ($lhs.clone()).ge($rhs.clone())
    };
    
    ($lhs:ident < $rhs:expr) => {
        ($lhs.clone()).gt($rhs.clone())
    };

    ($lhs:ident >= $rhs:expr) => {
        ($lhs.clone()).le($rhs.clone())
    };

    ($lhs:ident > $rhs:expr) => {
        ($lhs.clone()).lt($rhs.clone())
    };

    ($lhs:ident = $rhs:expr) => {
        ($lhs.clone()).sst($rhs.clone())
    };

    ( $lhs:ident || $rhs:expr ) => {
        $lhs.clone().lor($rhs.clone())
    };

    ( $lhs:ident && $rhs:expr ) => {
        $lhs.clone().land($rhs.clone())
    };
}

/**
  * Verilogモジュールクラス
  * すべてのASTの統合構造体
  **/
#[derive(Clone,Debug)]
pub struct VModule {
    Module_Name : String,
    IO_Port     : Vec<wrVar>,
    IO_Param    : Vec<wrVar>,
    Local       : Vec<wrVar>,
    Always_AST  : Vec<Always>,
    Assign_AST  : Vec<Assign>,
    Function_AST: Vec<Func_AST>,
    Fsm         : Vec<FsmModule>,
    Axi         : Vec<AXI>,
	Inline		: String,

    // generate code
    code        : String,
}

/*
/// 入出力ポート、内部配線用Trait
impl VModule{
    /// input の追加
    pub fn Input(&mut self, name: &str) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Input(name, 1);
        self.IO_Port.push(tmp.clone());
        return _V(tmp);
    }

    /// inout の追加
    pub fn Inout(&mut self, name: &str) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Inout(name, 1);
        self.IO_Port.push(tmp.clone());
        return _V(tmp);
    }

    /// output の追加
    pub fn Output(&mut self, name: &str) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Output(name, 1);
        self.IO_Port.push(tmp.clone());
        return _V(tmp);
    }

    /// output(register) の追加
    pub fn Reg_Output(&mut self, name: &str) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.OutputReg(name, 1);
        self.IO_Port.push(tmp.clone());
        return _V(tmp);
    }

    /// wire の追加
    pub fn Wire(&mut self, name: &str) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Wire(name, 1);
        self.Local.push(tmp.clone());
        return _V(tmp);
    }

    /// reg の追加
    pub fn Reg(&mut self, name: &str) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Reg(name, 1);
        self.Local.push(tmp.clone());
        return _V(tmp);
    }
}
*/

/// 入出力ポート、内部配線用Trait
pub trait Vset<T> {
    fn Input(&mut self, name: &str, Width: T) -> Box<E>;
    fn Inout(&mut self, name: &str, Width: T) -> Box<E>;
    fn Output(&mut self, name: &str, Width: T) -> Box<E>;
    fn Reg_Output(&mut self, name: &str, Width: T) -> Box<E>;
    fn Wire(&mut self, name: &str, Width: T) -> Box<E>;
    fn Reg(&mut self, name: &str, Width: T) -> Box<E>;
}

/// 入力幅：Box<E>
impl<T> Vset<T> for VModule
where
    T: Into<Box<E>>,
{
    /// input の追加
    fn Input(&mut self, name: &str, Width: T) -> Box<E> {
        let mut tmp = wrVar::new();
        let width = *Width.into();
        let len = if let E::Num(i) = width { i } else { 0 };
        tmp.Input(name, len);
        if let E::Ldc(wr) = width { tmp.Width( &( wr.getWP() ) ); };
        self.IO_Port.push(tmp.clone());
        return _V(tmp);
    }

    /// inout の追加
    fn Inout(&mut self, name: &str, Width: T) -> Box<E> {
        let mut tmp = wrVar::new();
        let width = *Width.into();
        let len = if let E::Num(i) = width { i } else { 0 };
        tmp.Inout(name, len);
        if let E::Ldc(wr) = width { tmp.Width( &( wr.getWP() ) ); };
        self.IO_Port.push(tmp.clone());
        return _V(tmp);
    }

    /// output の追加
    fn Output(&mut self, name: &str, Width: T) -> Box<E> {
        let mut tmp = wrVar::new();
        let width = *Width.into();
        let len = if let E::Num(i) = width { i } else { 0 };
        tmp.Output(name, len);
        if let E::Ldc(wr) = width { tmp.Width( &( wr.getWP() ) ); };
        self.IO_Port.push(tmp.clone());
        return _V(tmp);
    }

    /// output(register) の追加
    fn Reg_Output(&mut self, name: &str, Width: T) -> Box<E> {
        let mut tmp = wrVar::new();
        let width = *Width.into();
        let len = if let E::Num(i) = width { i } else { 0 };
        tmp.OutputReg(name, len);
        if let E::Ldc(wr) = width { tmp.Width( &( wr.getWP() ) ); };
        self.IO_Port.push(tmp.clone());
        return _V(tmp);
    }

    /// wire の追加
    fn Wire(&mut self, name: &str, Width: T) -> Box<E> {
        let mut tmp = wrVar::new();
        let width = *Width.into();
        let len = if let E::Num(i) = width { i } else { 0 };
        tmp.Wire(name, len);
        if let E::Ldc(wr) = width { tmp.Width( &( wr.getWP() ) ); };
        self.Local.push(tmp.clone());
        return _V(tmp);
    }

    /// reg の追加
    fn Reg(&mut self, name: &str, Width: T) -> Box<E> {
        let mut tmp = wrVar::new();
        let width = *Width.into();
        let len = if let E::Num(i) = width { i } else { 0 };
        tmp.Reg(name, len);
        if let E::Ldc(wr) = width { tmp.Width( &( wr.getWP() ) ); };
        self.Local.push(tmp.clone());
        return _V(tmp);
    }
}

impl VModule {
    /// モジュールの生成
    pub fn new(Name: &str) -> VModule {
        VModule{Module_Name: Name.to_string(), 
            IO_Port: Vec::new(),
            IO_Param: Vec::new(),
            Local: Vec::new(),
            Always_AST: Vec::new(),
            Assign_AST: Vec::new(),
            Function_AST: Vec::new(),
            Fsm: Vec::new(),
            Axi: Vec::new(),
			Inline: String::new(),
            
            code: " ".to_string()}
    }

    /// パラメータの追加
    pub fn Param(&mut self, name: &str, Value: i32) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Parameter(name, Value);
        self.IO_Param.push(tmp.clone());
        return _V(tmp);
    }

    /// ローカルパラメータの追加
    pub fn LParam(&mut self, name: &str, Value: i32) -> Box<E>{
        let mut tmp = wrVar::new();
        tmp.Parameter(name, Value);
        self.Local.push(tmp.clone());
        return _V(tmp);
    }

    /// Debug: モジュール名の取得
    fn getName(&mut self) -> String {
        self.Module_Name.clone()
    }

    /// always 構文ブロックの追加
    pub fn Always(&mut self, AST_of_Always: Always) {
        let tmp = AST_of_Always;
        self.Always_AST.push(tmp.clone());
        return;
    }

    /// assign 構文 AST の追加
    pub fn Assign(&mut self, AST_of_Assign: Assign) {
        let tmp = AST_of_Assign;
        self.Assign_AST.push(tmp.clone());
        return;
    }

	/*
    /// function 構文ブロックの追加
    pub fn Function(&mut self, AST_of_Function: Func_AST) {
        let tmp = AST_of_Function;
        self.Function_AST.push(tmp.clone());
	}
	*/

	/*
    /// FSM AST 構文ブロック群を追加
    pub fn FSM(&mut self, fsm: FsmModule) -> Box<E> {
		let self_fsm = self.Fsm.clone();
		for n in self_fsm {
			if _StrOut(n.clone().fsm) == _StrOut(fsm.clone().fsm) {
				panic!("Some name FSM exist. :{}\n", _StrOut(fsm.clone().fsm))
			}
		}
		
        let tmp = fsm.clone();
        let mut stmt = fsm.StateOut();
        let state = *(tmp.clone().StateReg());
        let p;
        let mut np = wrVar::new();
        let mut n = 0;
        for ss in &mut stmt {
            self.Local.push(wrVar{name: ss.getStateName(), 
                            io_param: io_p::param_, 
                            width: 0, 
                            length: 0, 
                            reg_set: false, 
                            value: n, 
                            width_p: "_".to_string(), 
                            length_p: "_".to_string()});
            n+=1;
        }

        if let E::Ldc(x) = state {
            p = x.clone();
            let nam = p.getName() + "_Next";
            if let E::Ldc(wr) = *wrVar::new().Reg(&nam, 32) {np = wr;}
        }
        else {return Box::new(E::Null);}
        self.Local.push(p);
        self.Local.push(np);
        self.Fsm.push(tmp.clone());

        return tmp.StateReg();
	}
	*/

    /// モジュールの AST 解析と Verilog 構文の出力
    pub fn endmodule(&mut self) -> String {
        let mut st = String::new();
        //print!("module {} ",self.getName());
        st += &format!("module {} ",self.getName());
        
        // 入出力パラメータ出力コード
        st += &PrintParam(self.IO_Param.clone());

        // 入出力ポート出力コード
        st += &PrintPort(self.IO_Port.clone());

        // 内部パラメータおよび内部配線出力コード
        st += &PrintLocal(self.Local.clone());

        // Assign構文出力コード
        st += &PrintAssign(self.Assign_AST.clone());

        // Always構文出力コード
        st += &PrintAlways(self.Always_AST.clone());

        // Function構文出力コード
        st += &PrintFunction(self.Function_AST.clone());

		if self.Fsm.len() != 0 || self.Axi.len() != 0 || self.Inline.len() != 0{
			st += "\n    // ----Extra Component Set----\n\n";

			// FSMの出力コード
        	if self.Fsm.clone().len() > 0 {
            	for tmp in self.Fsm.clone() {
                	st += &PrintFsm(tmp.clone());
            	}
        	}
        
        	if self.Axi.clone().len() > 0 {
            	let mut i = -1;
            	for tmp in self.Axi.clone() {
                	i += 1;
                	st += &PrintAXI(tmp.clone(), i);
            	}
        	}

			if self.Inline.len() > 0 {
				st += &self.Inline;
			}
		}

        st += "\nendmodule\n";
        self.code = st.clone();

        return st;
    }

    pub fn genPrint(&mut self) {
        println!("{}",self.code);
    }

    pub fn genFile(&mut self, path: &str) -> Result<(),Box<std::io::Error>> {
        //let mut file = File::create(path)?;
		let mut file = OpenOptions::new().write(true).create(true).open(path)?;
        write!(file, "{}", self.code)?;
        file.flush()?;
        Ok(())
    }

	/// Inline verilog 
	pub fn Inline(&mut self, code: &str) {
		self.Inline += code;
		self.Inline += "\n\n";
	}

	// debug
	pub fn get_mod_name(&mut self) -> String {
		self.Module_Name.clone()
	}

	pub fn out_port(&mut self) -> Vec<wrVar> {
		self.IO_Port.clone()
	}

	pub fn out_param(&mut self) -> Vec<wrVar> {
		self.IO_Param.clone()
	}

	pub fn out_l_param(&mut self) -> Vec<wrVar> {
		self.Local.clone()
	}

	pub fn out_func_name(&mut self) -> Vec<String> {
		let mut st = Vec::new();
		let tmp = self.Function_AST.clone();
		for x in tmp {
			let e = x.top;
			if let E::Ldc(wrtop) = (*e).clone() {
				st.push(wrtop.getName());
			}
		}
		st
	}

	pub fn out_assign(&mut self) -> Vec<Assign> {
		self.Assign_AST.clone()
	}

	pub fn out_always(&mut self) -> Vec<Always> {
		self.Always_AST.clone()
	}
}


/// function 構文ブロック追加用トレイト
#[allow(non_camel_case_types)]
pub trait Func_trait<T> {
	fn Function(&mut self, AST_of_Function: T);
}

#[allow(non_camel_case_types)]
impl Func_trait<Func_AST> for VModule {
	fn Function(&mut self, AST_of_Function: Func_AST) {
		self.Function_AST.push(AST_of_Function);
	}
}

#[allow(non_camel_case_types)]
impl Func_trait<&Func_AST> for VModule {
	fn Function(&mut self, AST_of_Function: &Func_AST) {
		self.Function_AST.push(AST_of_Function.clone());
	}
}

/// FSM 構文ブロック追加用トレイト
#[allow(non_camel_case_types)]
pub trait FSM_trait<T> {
	fn FSM(&mut self, fsm: T) -> Box<E>;
}

#[allow(non_camel_case_types)]
impl FSM_trait<FsmModule> for VModule {
	fn FSM(&mut self, fsm: FsmModule) -> Box<E> {
		let self_fsm = self.Fsm.clone();
		for n in self_fsm {
			if _StrOut(n.clone().fsm) == _StrOut(fsm.clone().fsm) {
				panic!("Some name FSM exist. :{}\n", _StrOut(fsm.clone().fsm))
			}
		}
		
        let tmp = fsm.clone();
        let mut stmt = fsm.StateOut();
        let state = *(tmp.clone().StateReg());
        let p;
        let mut np = wrVar::new();
        let mut n = 0;
        for ss in &mut stmt {
            self.Local.push(wrVar{name: ss.getStateName(), 
                            io_param: io_p::param_, 
                            width: 0, 
                            length: 0, 
                            reg_set: false, 
                            value: n, 
                            width_p: "_".to_string(), 
                            length_p: "_".to_string()});
            n+=1;
        }

        if let E::Ldc(x) = state {
            p = x.clone();
            let nam = p.getName() + "_Next";
            if let E::Ldc(wr) = *wrVar::new().Reg(&nam, 32) {np = wr;}
        }
        else {return Box::new(E::Null);}
        self.Local.push(p);
        self.Local.push(np);
        self.Fsm.push(tmp.clone());

        return tmp.StateReg();
    }
}

#[allow(non_camel_case_types)]
impl FSM_trait<&FsmModule> for VModule {
	fn FSM(&mut self, fsm: &FsmModule) -> Box<E> {
		let self_fsm = self.Fsm.clone();
		for n in self_fsm {
			if _StrOut(n.clone().fsm) == _StrOut(fsm.clone().fsm) {
				panic!("Some name FSM exist. :{}\n", _StrOut(fsm.clone().fsm))
			}
		}
		
		let tmp = fsm.clone();
		let retE = fsm.clone().StateReg();
        let mut stmt = fsm.clone().StateOut();
        let state = *(tmp.clone().StateReg());
        let p;
        let mut np = wrVar::new();
        let mut n = 0;
        for ss in &mut stmt {
            self.Local.push(wrVar{name: ss.getStateName(), 
                            io_param: io_p::param_, 
                            width: 0, 
                            length: 0, 
                            reg_set: false, 
                            value: n, 
                            width_p: "_".to_string(), 
                            length_p: "_".to_string()});
            n+=1;
        }

        if let E::Ldc(x) = state {
            p = x.clone();
            let nam = p.getName() + "_Next";
            if let E::Ldc(wr) = *wrVar::new().Reg(&nam, 32) {np = wr;}
        }
        else {return Box::new(E::Null);}
        self.Local.push(p);
        self.Local.push(np);
        self.Fsm.push(tmp);

        return retE;
    }
}

#[allow(non_camel_case_types)]
pub trait AXI_trait<T> {
    fn AXI(&mut self, setAXI: T);
}

#[allow(non_camel_case_types)]
impl AXI_trait<AXISLite> for VModule {
    fn AXI(&mut self, setAXI: AXISLite) {
        let length = self.Axi.len() as i32;
        
        let reg_length = setAXI.reg_array.len() as i32;
        let mut reg_addr_width: i32 = 1;

        // address width calc
        loop {
            if 2i32.pow(reg_addr_width as u32) >= (reg_length * 4 - 1) {
                break;
            }
            reg_addr_width += 1;
        }

        // read address channel
        let o_arr = self.Output(&(format!("o_s_arready{}", length.clone())), 0);
        let i_arv = self.Input(&(format!("i_s_arvalid{}", length.clone())), 0);
                    self.Input(&(format!("i_s_araddr{}", length.clone())), reg_addr_width);
                    self.Input(&(format!("i_s_arprot{}", length.clone())), 3);

        // read data channel
        let o_rda = self.Output(&(format!("o_s_rdata{}", length.clone())), 32);
        let o_rsp = self.Output(&(format!("o_s_rresp{}", length.clone())), 2);
        let o_rva = self.Output(&(format!("o_s_rvalid{}", length.clone())), 0);
        let i_rre = self.Input(&(format!("i_s_rready{}", length.clone())), 0);

        // write address channel
        let o_awr = self.Output(&(format!("o_s_awready{}", length.clone())), 0);
        let i_awv = self.Input(&(format!("i_s_awvalid{}", length.clone())), 0);
                    self.Input(&(format!("i_s_awaddr{}", length.clone())), reg_addr_width);
                    self.Input(&(format!("i_s_awprot{}", length.clone())), 3);

        // write data channel
        let i_wda = self.Input(&(format!("i_s_wdata{}", length.clone())), 32);
        let i_wst = self.Input(&(format!("i_s_wstrb{}", length.clone())), 4);
        let i_wva = self.Input(&(format!("i_s_wvalid{}", length.clone())), 0);
        let o_wre = self.Output(&(format!("o_s_wready{}", length.clone())), 0);

        // write response channel
        let o_bre = self.Output(&(format!("o_s_bresp{}", length.clone())), 2);
        let o_bva = self.Output(&(format!("o_s_bvalid{}", length.clone())), 0);
        let i_bre = self.Input(&(format!("i_s_bready{}", length.clone())), 0);

        // inner wire and register
        let r_arr = self.Reg(&(format!("r_arready{}", length.clone())), 0);
        let w_arv = self.Wire(&(format!("w_arvalid{}", length.clone())), 0);
                    self.Reg(&(format!("r_araddr{}", length.clone())), reg_addr_width);

        let r_rda = self.Reg(&(format!("r_rdata{}", length.clone())), 32);
        let r_rva = self.Reg(&(format!("r_rvalid{}", length.clone())), 0);
        let w_rre = self.Wire(&(format!("w_rready{}", length.clone())), 0);

        let r_awr = self.Reg(&(format!("r_awready{}", length.clone())), 0);
        let w_awv = self.Wire(&(format!("w_awvalid{}", length.clone())), 0);
                    self.Reg(&(format!("r_awaddr{}", length.clone())), reg_addr_width);

        let w_wda = self.Wire(&(format!("w_wdata{}", length.clone())), 32);
        let w_wst = self.Wire(&(format!("r_wstrb{}", length.clone())), 4);
        let w_wva = self.Wire(&(format!("w_wvalid{}", length.clone())), 0);
        let r_wre = self.Reg(&(format!("r_wready{}", length.clone())), 0);

        let r_bva = self.Reg(&(format!("r_bvalid{}", length.clone())), 0);
        let w_bre = self.Wire(&(format!("w_bready{}", length.clone())), 0);

        // 接続の追加
        self.Assign(o_arr._e(r_arr));
        self.Assign(w_arv._e(i_arv));

        self.Assign(o_rda._e(r_rda));
        self.Assign(o_rsp._e(0));
        self.Assign(o_rva._e(r_rva));
        self.Assign(w_rre._e(i_rre));

        self.Assign(o_awr._e(r_awr));
        self.Assign(w_awv._e(i_awv));
        //self.Assign(w_awa._e(i_awa));

        self.Assign(w_wda._e(i_wda));
        self.Assign(w_wst._e(i_wst));
        self.Assign(w_wva._e(i_wva));
        self.Assign(o_wre._e(r_wre));

        self.Assign(o_bre._e(0));
        self.Assign(o_bva._e(r_bva));
        self.Assign(w_bre._e(i_bre));

		for x in setAXI.reg_array.clone() {
			//println!("{:?}", (*x));
			if let E::Ldc(wr) = *x {
				self.Reg( &( wr.getName() ), wr.getWidth());
			};
		}

        self.Axi.push(AXI::Lite(setAXI));
    }
}

#[allow(non_camel_case_types)]
impl AXI_trait<AXIS> for VModule {
    fn AXI(&mut self, setAXI: AXIS) {
		let length = setAXI.length.clone();

		let mut addr_width: i32 = 1;
		loop {
        	if 2i32.pow(addr_width as u32) >= (length * 4 - 1) {
        	    break;
        	}
        	addr_width += 1;
		}

		// read address channel
		let i_rid = self.Input("i_saxi_arid", 0);
        let o_arr = self.Output("o_saxi_arready", 0);
        let i_arv = self.Input("i_saxi_arvalid", 0);
					self.Input("i_saxi_araddr", addr_width);
					self.Input("i_saxi_arlen", 8);
					self.Input("i_saxi_arburst", 2);
		
		// read data channel
		let o_rid = self.Output("o_saxi_rid", 0);
        let o_rda = self.Output("o_saxi_rdata", 32);
        let o_rsp = self.Output("o_saxi_rresp", 2);
        let o_rva = self.Output("o_saxi_rvalid", 0);
		let i_rre = self.Input("i_saxi_rready", 0);
		let o_rls = self.Output("o_saxi_rlast", 0);

		// write address channel
		let i_wid = self.Input("i_saxi_awid", 0);
        let o_awr = self.Output("o_saxi_awready", 0);
        let i_awv = self.Input("i_saxi_awvalid", 0);
					self.Input("i_saxi_awaddr", addr_width);
					self.Input("i_saxi_awlen", 8);
					self.Input("i_saxi_awburst", 2);

		// write data channel
        			self.Input("i_saxi_wdata", 32);
					self.Input("i_saxi_wstrb", 4);
		let i_wls =	self.Input("i_saxi_wlast", 0);
        let i_wva = self.Input("i_saxi_wvalid", 0);
		let o_wre = self.Output("o_saxi_wready", 0);
		
		// write response channel
		let o_bid = self.Output("o_saxi_bid", 0);
        let o_bre = self.Output("o_saxi_bresp", 2);
        let o_bva = self.Output("o_saxi_bvalid", 0);
		let i_bre = self.Input("i_saxi_bready", 0);


		// inner wire and register
		let r_awr = self.Reg("r_axi_awready", 0);
		let w_awv = self.Wire("w_axi_awvalid", 0);
					self.Reg("r_axi_awaddr", addr_width);
					self.Reg("r_axi_awlen", 8);

					self.Wire("w_axi_wdata", 32);
		let w_wls = self.Wire("w_axi_wlast", 0);
		let w_wva = self.Wire("w_axi_wvalid", 0);
		let r_wre = self.Reg("r_axi_wready", 0);

        let r_bva = self.Reg("r_axi_bvalid", 0);
		let w_bre = self.Wire("w_axi_bready", 0);

		let r_arr = self.Reg("r_axi_arready", 0);
		let w_arv = self.Wire("w_axi_arvalid", 0);
					self.Reg("r_axi_araddr", 32);
					self.Reg("r_axi_arlen", 8);

		let r_rda = self.Reg("r_axi_rdata", 32);
		let r_rva = self.Reg("r_axi_rvalid", 0);
		let w_rre = self.Wire("w_axi_rready", 0);
		let r_rls = self.Reg("r_axi_rlast", 0);
		if setAXI.clone().mem {
			self.Wire("axis_write", 32);
			self.Reg("axis_read", 32);
			self.Wire("axis_addr", 32);
			self.Wire("axis_wen", 0);
		}
		else {
			if let E::Null = *(setAXI.clone().rdata) {
				self.Wire("axis_read", 32);
			}
			self.Wire("axis_write", 32);
			self.Wire("axis_addr", 32);
			self.Wire("axis_wen", 0);
		}
		
		
		self.Assign(o_rid._e(i_rid));
		self.Assign(o_rsp._e(0));
		self.Assign(o_rda._e(r_rda));
		self.Assign(o_rva._e(r_rva));
		self.Assign(w_rre._e(i_rre));
		self.Assign(o_rls._e(r_rls));

		self.Assign(o_arr._e(r_arr));
		self.Assign(w_arv._e(i_arv));

		self.Assign(w_wls._e(i_wls));
		self.Assign(w_wva._e(i_wva));
		self.Assign(o_wre._e(r_wre));

		self.Assign(o_awr._e(r_awr));
		self.Assign(w_awv._e(i_awv));

		self.Assign(o_bva._e(r_bva));
		self.Assign(w_bre._e(i_bre));
		self.Assign(o_bid._e(i_wid));
		self.Assign(o_bre._e(0));

		self.Axi.push(AXI::Slave(setAXI));
	}
}


/// メモリレジスタ生成用のトレイト
#[allow(non_camel_case_types)]
pub trait Memset<T> {
    fn Mem(&mut self, name: &str, args: T) -> Box<E>;
}

/// 入力(Box<E>:Box<E>)生成するメモリ構文
impl<T, U> Memset<(T, U)> for VModule
where
    T: Into<Box<E>>,
    U: Into<Box<E>>,
{
    /// メモリ構文
    #[allow(non_camel_case_types)]
    fn Mem(&mut self, name: &str, args: (T, U)) -> Box<E> {
        let mut tmp = wrVar::new();
        tmp.Mem(name, 0, 0);
        if let E::Ldc(wr) = *args.0.into() { tmp.Width( &( wr.getName() ) ); };
        if let E::Ldc(wr) = *args.1.into() { tmp.Length( &( wr.getName() ) ); };
        self.Local.push(tmp.clone());
        return _V(tmp);
    }
}

/**
  * 入出力設定パラメータ
  * 特に大きな意味は無い
  **/
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum io_p {
    input_,
    output_,
    inout_,
    param_,
    none,
}

/// 入出力ポート、パラメータデータ格納構造体
/**
  * 入出力パラメータクラス
  * 
  **/
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub struct wrVar {
    name     : String,
    io_param : io_p,
    width    : i32,
    length   : i32,
    reg_set  : bool,
    value    : i32,
    width_p  : String,
    length_p : String,
}

/**
  * 入出力パラメータクラスメソッド
  * セット・ゲット・コピー関数
  **/
#[allow(non_camel_case_types)]
impl wrVar {
    /// コンストラクタ
    pub fn new() -> wrVar {
        wrVar{name: "None".to_string(), io_param: io_p::none, width: 0, length: 0, reg_set: false, value: 0, width_p: "_".to_string(), length_p: "_".to_string()}
    }

    /// データ取得メソッド:name
    pub fn getName(&self) -> String {
        self.name.clone()
    }

    /// データ取得メソッド:io_param
    pub fn getIO(&self) -> io_p {
        self.io_param.clone()
    }

    /// データ取得メソッド:width
    pub fn getWidth(&self) -> i32 {
        self.width.clone()
    }

    /// データ取得メソッド:length
    pub fn getLength(&self) -> i32 {
        self.length.clone()
    }

    /// データ取得メソッド:reg_set
    pub fn getReg(&self) -> bool {
        self.reg_set.clone()
    }

    /// データ取得メソッド:value
    pub fn getValue(&self) -> i32 {
        self.value.clone()
    }

    /// データ取得メソッド:width_p
    pub fn getWP(&self) -> String {
        self.width_p.clone()
    }

    /// データ取得メソッド:length_p
    pub fn getLP(&self) -> String {
        self.length_p.clone()
    }

    /// パラメータによる長さ設定メソッド
    pub fn Length(&mut self, S: &str) -> wrVar {
        self.length_p = S.to_string();
        self.clone()
    }

    /// パラメータによる幅設定メソッド
    pub fn Width(&mut self, S: &str) -> wrVar {
        self.width_p = S.to_string();
        self.clone()
    }

    /// パラメータ設定メソッド:input
    pub fn Input(&mut self, Name: &str, Width: i32) -> Box<E> {
        self.name = Name.to_string();
        self.width = Width;

        self.io_param = io_p::input_;
        _V(self.clone())
    }

    /// パラメータ設定メソッド:output
    pub fn Output(&mut self, Name: &str, Width: i32) -> Box<E> {
        self.name = Name.to_string();
        self.width = Width;

        self.io_param = io_p::output_;
        _V(self.clone())
    }

    /// パラメータ設定メソッド:inout
    pub fn Inout(&mut self, Name: &str, Width: i32) -> Box<E> {
        self.name = Name.to_string();
        self.width = Width;

        self.io_param = io_p::inout_;
        _V(self.clone())
    }

    /// パラメータ設定メソッド:output(register)
    pub fn OutputReg(&mut self, Name: &str, Width: i32) -> Box<E> {
        self.name = Name.to_string();

        self.io_param = io_p::output_;
        self.width = Width;
        self.reg_set = true;
        _V(self.clone())
    }

    /// パラメータ設定メソッド:parameter
    pub fn Parameter(&mut self, Name: &str, Value: i32) -> Box<E> {
        self.name = Name.to_string();
        self.value = Value;

        self.io_param = io_p::param_;
        _V(self.clone())
    }

    /// パラメータ設定メソッド:wire
    pub fn Wire(&mut self, Name: &str, Width: i32) -> Box<E> {
        self.name = Name.to_string();
        self.width = Width;

        _V(self.clone())
    }

    /// パラメータ設定メソッド:reg
    pub fn Reg(&mut self, Name: &str, Width: i32) -> Box<E> {
        self.name = Name.to_string();
        self.width = Width;

        self.reg_set = true;
        _V(self.clone())
    }

    /// パラメータ設定メソッド:reg[ length : 0 ]
    pub fn Mem(&mut self, Name: &str, Width: i32, Lenght: i32) -> Box<E> {
        self.name = Name.to_string();
        self.width = Width;
        self.length = Lenght;

        self.reg_set = true;
        _V(self.clone())
    }
}

/// Assign 構文代入用トレイト
pub trait SetEqual<T>
where
    T: Into<Box<E>>,
{
     fn _e(&self, RHS: T) -> Assign;

     fn _ve(&self, RHS: T) -> Assign;
}

/// Assign 構文代入用トレイト
impl<T> SetEqual<T> for Box<E>
where
    T: Into<Box<E>>,
{
    /// Box<E>からAssign生成を行うメソッド
    fn _e(&self, RHS: T) -> Assign {
        let mut tmp = Assign::new();
        tmp.L(self).R(&RHS.into())
    }

    fn _ve(&self, RHS: T) -> Assign {
        let mut tmp = Assign::new();
        tmp.L(self).R(&RHS.into())
    }
}

/**
  * assign構文用AST構造体
  * 
  **/
#[derive(Clone,Debug)]
pub struct Assign {
    lhs     : Box<E>,
    rhs     : Box<E>,
}

/**
  * assign構文用ASTメソッド
  * 
  **/
impl Assign {
    /// assign 構文生成
    pub fn new() -> Assign {
        Assign{lhs: Box::new(E::Ldc(wrVar::new())), rhs: Box::new(E::Ldc(wrVar::new()))}
    }

    /// 左辺設定メソッド
    pub fn L<T: Into<Box<E>>>(&mut self, LHS: T) -> Assign {
        self.lhs = LHS.into();
        let tmp = self.clone();
        return tmp;
    }

    /// 右辺設定メソッド
    pub fn R<T: Into<Box<E>>>(&mut self, RHS: T) -> Assign {
        self.rhs = RHS.into();
        let tmp = self.clone();
        return tmp;
    }

    /// 左辺出力メソッド
    pub fn LOut(&mut self) -> Box<E> {
        self.lhs.clone()
    }

    /// 右辺出力メソッド
    pub fn ROut(&mut self) -> Box<E> {
        self.rhs.clone()
    }
}

/**
  * Always構文用AST構造体
  * 
  **/
#[derive(Clone,Debug)]
pub struct Always{
    br      : String,
    stmt    : Vec<Box<E>>,
    P_edge  : Vec<wrVar>,
    N_edge  : Vec<wrVar>,
}

/// Always構文内使用の立ち上がり信号設定構文
pub fn Posedge<T: Into<Box<E>>>(edge: T) -> Always {
    let e = *edge.into();
    let mut tmp = Always{br: "brock".to_string(), stmt: Vec::new(), P_edge: Vec::new(), N_edge: Vec::new()};
    match e {
        E::Ldc(wr) => tmp.P_edge.push(wr.clone()),
        _ => return tmp,
    }
    tmp.clone()
}

/// Always構文内使用の立ち下り信号設定構文
pub fn Negedge<T: Into<Box<E>>>(edge: T) -> Always {
    let e = *edge.into();
    let mut tmp = Always{br: "brock".to_string(), stmt: Vec::new(), P_edge: Vec::new(), N_edge: Vec::new()};
    match e {
        E::Ldc(wr) => tmp.N_edge.push(wr.clone()),
        _ => return tmp,
    }
    tmp.clone()
}

/// Always構文内使用の信号未設定構文
pub fn Nonedge() -> Always {
    Always{br: "brock".to_string(), stmt: Vec::new(), P_edge: Vec::new(), N_edge: Vec::new()}
}

/**
  * Always構文用ASTメソッド
  * 
  **/
impl Always {
    // Debug
    pub fn new() -> Always {
        Always{br: "brock".to_string(), stmt: Vec::new(), P_edge: Vec::new(), N_edge: Vec::new()}
    }

    /// debug:外部出力
    fn blockout(&mut self) ->String {
        self.br.clone()
    }

    /// ブロッキング設定
    pub fn block(&mut self)-> Always {
        self.br = "brock".to_string();
        self.clone()
    }

    /// ノンブロッキング設定
    pub fn non(&mut self)-> Always {
        self.br = "Non".to_string();
        self.clone()
    }

    /// 立ち上がり信号設定
    pub fn Posedge<T: Into<Box<E>>>(&mut self, edge: T) -> Always {
        let e = *edge.into();
        match e {
            E::Ldc(wr) => self.P_edge.push(wr.clone()),
            _ => return self.clone(),
        }
        self.clone()
    }

    /// 立ち下がり信号設定
    pub fn Negedge<T: Into<Box<E>>>(&mut self, edge: T) -> Always {
        let e = *edge.into();
        match e {
            E::Ldc(wr) => self.N_edge.push(wr.clone()),
            _ => return self.clone(),
        }
        self.clone()
    }

    /// 分岐 if 構文追加
    pub fn If<T: Into<Box<E>>>(&mut self, C: T, S: Vec<Box<E>>) -> Always {
        let i = If(C.into(), S);
        self.stmt.push(i);
        self.clone()
    }

    /// 分岐 else if 構文追加
    pub fn Else_If<T: Into<Box<E>>>(&mut self, C: T, S: Vec<Box<E>>) -> Always {
        let n = self.stmt.pop().unwrap();
        let mut p;
        let e = *n;
        match e {
            E::BL(n) => {
                p = n.clone();
                p.push(IfStmt_AST{If_: true, IfE: C.into(), ST: S});
                self.stmt.push(Box::new(E::BL(p)));
            },
            _ => {return self.clone();},
        }
        self.clone()
    }

    /// 分岐 else 構文追加
    pub fn Else(&mut self, S: Vec<Box<E>>) -> Always {
        let n = self.stmt.pop().unwrap();
        let mut p;
        let e = *n;
        match e {
            E::BL(n) => {
                p = n.clone();
                p.push(IfStmt_AST{If_: false, IfE: Box::new(E::Null), ST: S});
                self.stmt.push(Box::new(E::BL(p)));
            },
            _ => {},
        }
        self.clone()
    }

    /// Case文追加
    pub fn Case<T: Into<Box<E>>>(&mut self, Sel: T) -> Always {
        let c = Case(Sel.into());
        self.stmt.push(c);
        self.clone()
    }

    /// Case文内の分岐追加
    pub fn S<T: Into<Box<E>>>(&mut self, C: T, S: Vec<Box<E>>) -> Always {
        let c = self.stmt.pop().unwrap();
        let mut p;
        let cs = *c;
        match cs {
            E::CS(tm) => {
                p = tm.clone();
                p.SetCaseS(C.into(), S);
                self.stmt.push(Box::new(E::CS(p)))
            },
            _ => {
                println!("abort");
                panic!("Not Case");
            },
        }
        self.clone()
    }

	/// Case文内のデフォルト追加
    pub fn Default(&mut self, S: Vec<Box<E>>) -> Always {
        let c = self.stmt.pop().unwrap();
        let mut p;
        let cs = *c;
        match cs {
            E::CS(tm) => {
                p = tm.clone();
                p.SetCaseS(Box::new(E::Null), S);
                self.stmt.push(Box::new(E::CS(p)))
            },
            _ => {
                println!("abort");
                panic!("Not Case");
            },
        }
        self.clone()
	}
	
	pub fn out_p_edge(&mut self) -> Vec<wrVar> {
		self.P_edge.clone()
	}

	pub fn out_n_edge(&mut self) -> Vec<wrVar> {
		self.N_edge.clone()
	}
}

/**
  * function生成用関数
  *
  **/ 
#[allow(non_camel_case_types)]
pub fn func(name: &str, width: i32) -> Func_AST {
    Func_AST{top: wrVar::new().Wire(name, width), input: Vec::new(), stmt: Vec::new()}
}

/**
  * function構文用AST構造体
  * 
  **/
#[allow(non_camel_case_types)]
#[derive(Clone,Debug)]
pub struct Func_AST {
    top   : Box<E>,
    input : Vec<Box<E>>,
    stmt  : Vec<Box<E>>,
}

/**
  * function引数設定マクロ
  * 
  **/
#[macro_export]
macro_rules! func_args {
    ( $($x: expr),* ) => (
        {let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x.clone());
        )*
        temp_vec
		}
    )
}

/**
  * function構文用ASTメソッド
  * 
  **/
impl Func_AST {
    /// Functionのトップ文字列を格納したAST取得
    pub fn own(&mut self) -> Box<E> {
        self.top.clone()
    }

    /// debug:構文生成
    pub fn using(&mut self, args: Vec<Box<E>>) -> Box<E> {
        let tmp = Box::new(E::Func(self.top.clone(), args));
        tmp.clone()
    }

    /// 入力の追加
    pub fn Input(&mut self, Name: &str, Width: i32) -> Box<E> {
        let mut tmp = wrVar::new();
        let port = tmp.Input(Name, Width);
        self.input.push(port.clone());
        port
    }

    /// 分岐 if 構文追加
    pub fn If<T: Into<Box<E>>>(&mut self, C: T, S: Vec<Box<E>>) -> Func_AST {
        let i = If(C.into(), S);
        self.stmt.push(i);
        self.clone()
    }

    /// 分岐 else if 構文追加
    pub fn Else_If<T: Into<Box<E>>>(&mut self, C: T, S: Vec<Box<E>>) -> Func_AST {
        let n = self.stmt.pop().unwrap();
        let mut p;
        let e = *n;
        match e {
            E::BL(n) => {
                p = n.clone();
                p.push(IfStmt_AST{If_: true, IfE: C.into(), ST: S});
                self.stmt.push(Box::new(E::BL(p)));
            },
            _ => {return self.clone();},
        }
        self.clone()
    }

    /// 分岐 else 構文追加
    pub fn Else(&mut self, S: Vec<Box<E>>) -> Func_AST {
        let n = self.stmt.pop().unwrap();
        let mut p;
        let e = *n;
        match e {
            E::BL(n) => {
                p = n.clone();
                p.push(IfStmt_AST{If_: false, IfE: Box::new(E::Null), ST: S});
                self.stmt.push(Box::new(E::BL(p)));
            },
            _ => {},
        }
        self.clone()
    }

    /// Case 文追加
    pub fn Case<T: Into<Box<E>>>(&mut self, Sel: T) -> Func_AST {
        let c = Case(Sel.into());
        self.stmt.push(c);
        self.clone()
    }

    /// Case 文内の分岐追加
    pub fn S<T: Into<Box<E>>>(&mut self, C: T, S: Vec<Box<E>>) -> Func_AST {
        let c = self.stmt.pop().unwrap();
        let mut p;
        let cs = *c;
        match cs {
            E::CS(tm) => {
                p = tm.clone();
                p.SetCaseS(C.into(), S);
                self.stmt.push(Box::new(E::CS(p)))
            },
            _ => {
                println!("abort");
            },
        }
        self.clone()
    }

    /// Case 文のデフォルト追加
    pub fn Default(&mut self, S: Vec<Box<E>>) -> Func_AST {
        let c = self.stmt.pop().unwrap();
        let mut p;
        let cs = *c;
        match cs {
            E::CS(tm) => {
                p = tm.clone();
                p.SetCaseS(Box::new(E::Null), S);
                self.stmt.push(Box::new(E::CS(p)))
            },
            _ => {
                println!("abort");
				panic!("Not Case");
            },
        }
        self.clone()
    }
}

/**
  * if,else if,else構造体
  * 
  **/
#[allow(non_camel_case_types)]
#[derive(Clone,Debug)]
pub struct IfStmt_AST {
    If_     : bool,         // if文フラグ
    IfE     : Box<E>,       // if文条件式
    ST      : Vec<Box<E>>,  // 実行式
}

impl IfStmt_AST {
    fn getIfFlag(&mut self) -> bool {
        self.If_.clone()
    }

    fn getTerms(&mut self) -> Box<E> {
        self.IfE.clone()
    }

    fn getStatement(&mut self) -> Vec<Box<E>> {
        self.ST.clone()
    }
}

/// ステートメントブロック内のif構文作成
#[allow(non_camel_case_types)]
pub fn If<T: Into<Box<E>>>(C: T, S: Vec<Box<E>>) -> Box<E> {
    let mut i = Vec::new();
    i.push(IfStmt_AST{If_: true, IfE: C.into(), ST: S});
    Box::new(E::BL(i))
}

/// ステートメントブロック内のif分岐追加
#[allow(non_camel_case_types)]
pub trait Ifset {
    #[allow(non_camel_case_types)]
    fn Else_If<T: Into<Box<E>>>(self, C: T, S: Vec<Box<E>>) -> Box<E>;

    #[allow(non_camel_case_types)]
    fn Else(self, S: Vec<Box<E>>) -> Box<E>;
}


impl Ifset for Box<E> {
    #[allow(non_camel_case_types)]
    fn Else_If<T: Into<Box<E>>>(self, C: T, S: Vec<Box<E>>) -> Box<E> {
        let e = *self;
        let mut p;
        match e {
            E::BL(n) => {
                p = n.clone();
                p.push(IfStmt_AST{If_: true, IfE: C.into(), ST: S});
            },
            _ => return Box::new(E::Null),
        }
        return Box::new(E::BL(p));
    }

    #[allow(non_camel_case_types)]
    fn Else(self, S: Vec<Box<E>>) -> Box<E> {
        let e = *self;
        let mut p;
        match e {
            E::BL(n) => {
                p = n.clone();
                p.push(IfStmt_AST{If_: false, IfE: Box::new(E::Null), ST: S});
            },
            _ => return Box::new(E::Null),
        }
        return Box::new(E::BL(p));
    }
}

/**
  * Case構造体
  * 
  **/
#[allow(non_camel_case_types)]
#[derive(Clone,Debug)]
pub struct CaseStmt_AST {
    CaseVar : wrVar,
    Select  : Vec<Case_>,
}

impl CaseStmt_AST {
    pub fn SetCaseV(&mut self, V: wrVar) {
        self.CaseVar = V.clone()
    }

    pub fn SetCaseS<T: Into<Box<E>>>(&mut self, Cond: T, Stmt: Vec<Box<E>>) {
        self.Select.push(Case_{CaseT: Cond.into(), CaseS: Stmt})
    }

    pub fn getCaseV(&mut self) -> wrVar {
        self.CaseVar.clone()
    }

    pub fn getSelect(&mut self) -> Vec<Case_> {
        self.Select.clone()
    }
}

// ステートメントブロック内のcase文作成
#[allow(non_camel_case_types)]
fn Case<T: Into<Box<E>>>(Sel: T) -> Box<E> {
    let e = *Sel.into();
    let mut C = CaseStmt_AST{CaseVar: wrVar::new(), Select: Vec::new()};
    match e {
        E::Ldc(wr) => {
            C.SetCaseV(wr);
        },
        _ => {
            Box::new(E::Null);
        },
    }

    Box::new(E::CS(C))
}

// ステートメントブロック内のcase分岐追加
#[allow(non_camel_case_types)]
pub trait Caseset {
    #[allow(non_camel_case_types)]
    fn S<T: Into<Box<E>>>(self, C: T, S: Vec<Box<E>>) -> Box<E>;

    #[allow(non_camel_case_types)]
    fn Default(self, S: Vec<Box<E>>) -> Box<E>;
}

#[allow(non_camel_case_types)]
impl Caseset for Box<E> {
    #[allow(non_camel_case_types)]
    fn S<T: Into<Box<E>>>(self, C: T, S: Vec<Box<E>>) -> Box<E> {
        let e = *self;
        let mut n;
        match e {
            E::CS(csast) => {
                n = csast;
            },
            _ => return Box::new(E::Null),
        }
        n.SetCaseS(C.into(), S);
        Box::new(E::CS(n))
    }

    #[allow(non_camel_case_types)]
    fn Default(self, S: Vec<Box<E>>) -> Box<E> {
        let e = *self;
        let mut n;
        match e {
            E::CS(csast) => {
                n = csast;
            },
            _ => return Box::new(E::Null),
        }
        n.SetCaseS(Box::new(E::Null), S);
        Box::new(E::CS(n))
    }
}


/**
  *　Caseの各条件における内部構造体
  * 
  **/
#[derive(Clone,Debug)]
pub struct Case_ {
    pub CaseT   : Box<E>,
    pub CaseS   : Vec<Box<E>>,
}

/// ステートメントブロック用ベクタ_ブロック作成 & 式追加
pub fn Form<T: Into<Box<E>>>(formu: T) -> Vec<Box<E>> {
    let mut tmp = Vec::new();
    tmp.push(formu.into());
    return tmp
}

/// ステートメントブロック内の式追加
#[allow(non_camel_case_types)]
pub trait addForm<T>
where
    T: Into<Box<E>>,
{
     fn Form(self, formu: T) -> Vec<Box<E>>;
}

impl<T> addForm<T> for Vec<Box<E>>
where
    T: Into<Box<E>>,
{
    fn Form(self, formu: T) -> Vec<Box<E>> {
        let mut tmp = self;
        tmp.push(formu.into());
        return tmp
    }
}

/**
  * 各構文用列挙型構造体
  * 
  **/
#[derive(Clone,Debug)]
pub enum E {
    Null,
    Ldc(wrVar),                     // 変数
    Num(i32),                       // 数値
    No(Box<E>),                     // Not構文
    Red(String, Box<E>),            // リダクション構文
    Bin(String, Box<E>, Box<E>),    // 二項演算
    PL(Box<E>, Box<E>, Box<E>),     // 分岐構文
    SB(Box<E>, Box<E>),             // 代入文
    CS(CaseStmt_AST),               // case文
    BL(Vec<IfStmt_AST>),            // if, else if, else文
    Func(Box<E>, Vec<Box<E>>),      // function文
    MEM(Box<E>,Box<E>),             // メモリ
	MBT(Box<E>,Box<E>,Box<E>),		// 多ビット
    Node(String),                   // 内部検索用
}

impl<'a> From<&'a Box<E>> for Box<E> {
    fn from(x: &'a Box<E>) -> Self {
        x.clone()
    }
}

impl<'a> From<&'a mut Box<E>> for Box<E> {
    fn from(x: &'a mut Box<E>) -> Self {
        x.clone()
    }
}

impl From<i32> for Box<E> {
    fn from(i: i32) -> Self {
        _Num(i)
    }
}

impl From<&i32> for Box<E> {
    fn from(i: &i32) -> Self {
        _Num(*i)
    }
}

// 変数出力関数
fn _V(V: wrVar) -> Box<E>{
    Box::new(E::Ldc(V))
}

// 数値出力関数
pub fn _Num(num: i32) -> Box<E>{
    Box::new(E::Num(num))
}

// 代入演算関数
pub fn _Veq<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::SB(L.into(), R.into()))
}

// 分岐構文関数
pub fn _Branch<T: Into<Box<E>>, U: Into<Box<E>>, V: Into<Box<E>>>(
    Terms: T,
    TrueNode: U,
    FalseNode: V,
) -> Box<E> {
    Box::new(E::PL(Terms.into(), TrueNode.into(), FalseNode.into()))
}

// 演算子関数
/// "+" addition
fn _Add<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("add".to_string(), L.into(), R.into()))
}

/// "-" substruction
fn _Sub<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("sub".to_string(), L.into(), R.into()))
}

/// "*" multipication
fn _Mul<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("mul".to_string(), L.into(), R.into()))
}

/// "/" division
fn _Div<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("div".to_string(), L.into(), R.into()))
}

/// "%" modulo
fn _Mod<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("mod".to_string(), L.into(), R.into()))
}

/// "||" or
fn _LOr<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("lor".to_string(), L.into(), R.into()))
}

/// "&&" and
fn _LAnd<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("land".to_string(), L.into(), R.into()))
}

/// "|" or
fn _Or<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("or".to_string(), L.into(), R.into()))
}

/// "&" and
fn _And<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("and".to_string(), L.into(), R.into()))
}

/// "^" exclusive or
fn _Xor<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("xor".to_string(), L.into(), R.into()))
}

/// "==" equal
pub fn _Eq<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("equal".to_string(), L.into(), R.into()))
}

/// "!=" not equal
pub fn _Neq<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("Not equal".to_string(), L.into(), R.into()))
}

/// "<<" left shift
fn _LSH<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("lshift".to_string(), L.into(), R.into()))
}

/// ">>" right shift
fn _RSH<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("rshift".to_string(), L.into(), R.into()))
}

/// ">>>" right arithmetic shift
pub fn _RSHA<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("rshifta".to_string(), L.into(), R.into()))
}

/// "<" more than
fn _MTH<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("more_than".to_string(), L.into(), R.into()))
}

/// ">" less than
fn _LTH<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("less_than".to_string(), L.into(), R.into()))
}

/// "<=" or more
fn _OMR<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("or_more".to_string(), L.into(), R.into()))
}

/// ">=" or less
fn _OLS<T: Into<Box<E>>, U: Into<Box<E>>>(L: T, R: U) -> Box<E> {
    Box::new(E::Bin("or_less".to_string(), L.into(), R.into()))
}

/**
  * 演算子実装メソッド
  *
  **/
pub trait Notc {
    fn not(&self) -> Box<E>;
}

impl Notc for Box<E> {
    fn not(&self) -> Box<E> {
        Box::new(E::No(self.clone()))
    }
}

impl Not for Box<E> {
    type Output = Box<E>;

    fn not(self) -> Box<E> {
        Box::new(E::No(self.clone()))
    }
}

impl<T> Add<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn add(self, other: T) -> Box<E> {
        _Add(self, other.into())
    }
}

impl<T> Add<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn add(self, other: T) -> Box<E> {
        _Add(self, &other.into())
    }
}

impl<T> Sub<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn sub(self, other: T) -> Box<E> {
        _Sub(self, other.into())
    }
}

impl<T> Sub<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn sub(self, other: T) -> Box<E> {
        _Sub(self, &other.into())
    }
}

impl<T> Mul<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn mul(self, other: T) -> Box<E> {
        _Mul(self, other.into())
    }
}

impl<T> Mul<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn mul(self, other: T) -> Box<E> {
        _Mul(self, &other.into())
    }
}

impl<T> Div<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn div(self, other: T) -> Box<E> {
        _Div(self, other.into())
    }
}

impl<T> Div<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn div(self, other: T) -> Box<E> {
        _Div(self, &other.into())
    }
}

impl<T> Rem<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn rem(self, other: T) -> Box<E> {
        _Mod(self, other.into())
    }
}

impl<T> Rem<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn rem(self, other: T) -> Box<E> {
        _Mod(self, &other.into())
    }
}

impl<T> BitOr<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn bitor(self, other: T) -> Box<E> {
        _Or(self, other.into())
    }
}

impl<T> BitOr<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn bitor(self, other: T) -> Box<E> {
        _Or(self, &other.into())
    }
}

impl<T> BitAnd<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn bitand(self, other: T) -> Box<E> {
        _And(self, other.into())
    }
}

impl<T> BitAnd<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn bitand(self, other: T) -> Box<E> {
        _And(self, &other.into())
    }
}

impl<T> BitXor<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn bitxor(self, other: T) -> Box<E> {
        _Xor(self, other.into())
    }
}

impl<T> BitXor<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn bitxor(self, other: T) -> Box<E> {
        _Xor(self, &other.into())
    }
}

impl<T> Shl<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn shl(self, other: T) -> Box<E> {
        _LSH(self, other.into())
    }
}

impl<T> Shl<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn shl(self, other: T) -> Box<E> {
        _LSH(self, &other.into())
    }
}

impl<T> Shr<T> for Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn shr(self, other: T) -> Box<E> {
        _RSH(self, other.into())
    }
}

impl<T> Shr<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    type Output = Box<E>;

    fn shr(self, other: T) -> Box<E> {
        _RSH(self, &other.into())
    }
}

// Equal,Not Equal構文生成
pub trait PartialEq<Rhs = Self> {

    fn eq(self, other: Rhs) -> Box<E>;

    fn ne(self, other: Rhs) -> Box<E>;
}

impl<T> PartialEq<T> for Box<E>
where
    T: Into<Box<E>>,
{
    fn eq(self, other: T) -> Box<E> {
        _Eq(self, other.into())
    }

    fn ne(self, other: T) -> Box<E> {
        _Neq(self, other.into())
    }
}

impl<T> PartialEq<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    fn eq(self, other: T) -> Box<E> {
        _Eq(self, &other.into())
    }

    fn ne(self, other: T) -> Box<E> {
        _Neq(self, &other.into())
    }
}

// compare構文生成
pub trait PartialOrd<Rhs = Self>{
    fn lt(self, other: Rhs) -> Box<E>;

    fn le(self, other: Rhs) -> Box<E>;

    fn gt(self, other: Rhs) -> Box<E>;

    fn ge(self, other: Rhs) -> Box<E>;
}

impl<T> PartialOrd<T> for Box<E>
where
    T: Into<Box<E>>,
{
    fn lt(self, other: T) -> Box<E> {
        _LTH(self, other.into())
    }

    fn le(self, other: T) -> Box<E> {
        _OLS(self, other.into())
    }

    fn gt(self, other: T) -> Box<E> {
        _MTH(self, other.into())
    }

    fn ge(self, other: T) -> Box<E> {
        _OMR(self, other.into())
    }
}

impl<T> PartialOrd<T> for &Box<E>
where
    T: Into<Box<E>>,
{
    fn lt(self, other: T) -> Box<E> {
        _LTH(self, &other.into())
    }

    fn le(self, other: T) -> Box<E> {
        _OLS(self, &other.into())
    }

    fn gt(self, other: T) -> Box<E> {
        _MTH(self, &other.into())
    }

    fn ge(self, other: T) -> Box<E> {
        _OMR(self, &other.into())
    }
}

// 代入文生成
pub trait Subs<Rhs = Self> {
    fn sst(&self, other: Rhs) -> Box<E>;
}

impl<T> Subs<T> for Box<E>
where
    T: Into<Box<E>>,
{
    fn sst(&self, other: T) -> Box<E> {
        _Veq(self.clone(), other.into())
    }
}

// 論理演算子生成
pub trait Logi<Rhs = Self> {
    fn land(&self, other: Rhs) -> Box<E>;

    fn lor(&self, other: Rhs) -> Box<E>;
}

impl<T> Logi<T> for Box<E>
where
    T: Into<Box<E>>,
{
    fn land(&self, other: T) -> Box<E> {
        _LAnd(self, &other.into())
    }

    fn lor(&self, other: T) -> Box<E> {
        _LOr(self, &other.into())
    }
}

// メモリ、レジスタ用アドレス指定
pub trait Addr<Rs = Self> {
    fn addr(&self, address: Rs) ->Box<E>;
}

impl<T> Addr<T> for Box<E>
where
    T: Into<Box<E>>,
{
    fn addr(&self, address: T) -> Box<E> {
        Box::new(E::MEM(self.clone(), address.into()))
    }
}

// レジスタ用多ビット指定
pub trait MBit<Rs = Self> {
    fn range(&self, hbit: Rs, lbit: Rs) ->Box<E>;
}

impl<T> MBit<T> for Box<E>
where
    T: Into<Box<E>>,
{
    fn range(&self, hbit: T, lbit: T) -> Box<E> {
        Box::new(E::MBT(self.clone(), hbit.into(), lbit.into()))
    }
}

/**
  * 出力、分解、デバッグ関数
  * 出力関数以外はデバッグ用関数のため削除しても問題はない
  **/

/// 分解出力関数
fn DecompAST(Parenthesis: bool, ast: Box<E>, cnfg: &str, indent: i32) -> String{
    let e = *ast;
    let mut st = String::new();

    match e {
        E::Bin(ref bin, ref l, ref r) => {
            let tmp = bin.as_str();
            for _ in 0..indent {
                st += "    ";
            }
            if Parenthesis {
                match tmp.clone() {
                    "add" => {st += "("},
                    "sub" => {st += "("},
                    "or" => {st += "("},
                    "lor" => {st += "("},
                    _ => {st += ""},
                }
            }
            let mut pareset = false;
            st += &DecompAST(false ,l.clone(),cnfg, 0);
            match tmp.clone() {
                "add" => {st += "+";},
                "sub" => {st += "-";},
                "mul" => {st += "*"; pareset = true},
                "div" => {st += "/"; pareset = true},
                "mod" => {st += "%"; pareset = true},
                "or"  => {st += "|";},
                "and" => {st += "&";},
				"xor" => {st += "^";},
                "lor"  => {st += "||";},
                "land" => {st += "&&";},
                "lshift" => {st += "<<";},
                "rshift" => {st += ">>";},
				"rshifta" => {st += ">>>";},
                "equal" => {st += "==";},
                "Not equal" => {st += "!=";},
                "more_than" => {st += "<";},
                "less_than" => {st += ">";},
                "or_more" => {st += "<=";},
                "or_less" => {st += ">=";},
                _ => panic!("No correspond syntax : error operator -- {}", tmp),
            }
            st += &DecompAST(pareset, r.clone(),cnfg, 0);
            if Parenthesis {
                match tmp {
                    "add" => {st += ")";},
                    "sub" => {st += ")";},
                    "or" => {st += ")";},
                    "lor" => {st += ")";},
                    _ => {st += "";},
                }
            }
        }
        E::Ldc(ref wr) => {
            st += &format!("{}",wr.getName());
        }
        E::Num(ref i) => {
            st += &format!("{}",i);
        }
        E::PL(ref d, ref t, ref f) => {
            st += "(";
            st += &DecompAST(false,d.clone(),cnfg, 0);
            st += ")? ";
            st += &DecompAST(false, t.clone(),cnfg, 0);
            st += ": ";

            st += &DecompAST(false, f.clone(),cnfg, 0);
        },
        E::SB(ref l, ref r) => {
            for _ in 0..indent {
                st += "    ";
            }
            st += &DecompAST(false, l.clone(),cnfg, indent);
            if cnfg.to_string() == "brock".to_string() {
                st += " = ";
            }
            else {
                st += " <= ";
            }
            st += &DecompAST(false, r.clone(),cnfg, 0);
            st += ";\n";
        }
        E::CS(ref c) => {
            let cn = &*c;
            st += &PrintCase(cn.clone(),cnfg, indent);
        }
        E::BL(ref i) => {
            let iels = &*i;
            st += &PrintIf(iels.clone(),cnfg, indent);
        }
        E::MEM(ref m, ref a) => {
            let ma = &*m;
            let aa = &*a;
            st += &DecompAST(false, ma.clone(),cnfg, indent);
            st += &format!("[");
            st += &DecompAST(false, aa.clone(),cnfg, 0);
            st += &format!("]");
        }
		E::MBT(ref m, ref a, ref b) => {
			let mn = &*m;
			let aa = &*a;
			let bb = &*b;
			st += &DecompAST(false, mn.clone(),cnfg, indent);
			st += &format!("[");
            st += &DecompAST(false, aa.clone(),cnfg, 0);
			st += &format!(":");
			st += &DecompAST(false, bb.clone(),cnfg, 0);
            st += &format!("]");
		}
		E::Func(ref a, ref v) => {
			st += &DecompAST(false, a.clone(), cnfg, 0);
			st += &format!("(");
			let mut i: usize = 0;
			for x in v.clone() {
				st += &DecompAST(false, x.clone(), cnfg, 0);
				i += 1;
				if v.len() != i {
					st += &format!(", ");
				}
			}
			st += &format!(")");
		},
        E::No(ref b) => {
            let bb = &*b;
            st += "~";
            st += &DecompAST(false, bb.clone(),cnfg, 0);
        }
        E::Red(ref r, ref a) => {
            let tmp = r.as_str();
            match tmp.clone() {
                "and" => {st += "&";},
                "or"  => {st += "|"},
                "xor" => {st += "^"},
                "nand"=> {st += "~&"},
                "nor" => {st += "~|"},
                "xnor"=> {st += "~^"},
				_ => {return st;},
            }
			st += &DecompAST(false, a.clone(), cnfg, 0);
        }
        _ => {
            st += "";
        }
    }
    return st;
}

/// GlobalParameter出力関数
fn PrintParam(Param: Vec<wrVar>) -> String {
    let tmp = Param;
    let n = tmp.len();
    let mut num = 0;
    let mut st = String::new();
    if n != 0 {
        st += "#(\n";
    }

    for x in tmp {
        num += 1;
        st += &format!("    parameter {} = {}",x.getName(), x.getValue());
        if n > num {
            st += ",\n";
        }
        else {
            st += "\n";
        }
    }
    if n != 0 {
        st += ")\n";
    }
    return st;
}

/// 入出力ポート出力関数
fn PrintPort(Port: Vec<wrVar>) -> String {
    let tmp = Port;
    let n = tmp.len();
    let mut num = 0;

    let mut st = String::new();

    st += "(\n";
    //println!("(");
        for x in tmp {
            let port_set = x.getIO();
            num += 1;
            match port_set {
                io_p::input_ => {
                    st += "    input ";
                }
                io_p::output_ => {
                    if x.getReg() {
                        st += "    output reg ";
                    }
                    else {
                        st += "    output ";
                    }
                }
                io_p::inout_ => {
                    st += "    inout ";
                }
                _ => return st
            }

            if x.getWidth() == 0 && x.getWP() != "_" {
                st += &format!("[{}-1:0] ",x.getWP());
            }
            else if x.getWidth() > 1 {
                st += &format!("[{}:0] ",x.getWidth()-1);
            }
            else {
                st += " ";
            }

            st += &format!("{}",x.getName());

            if x.getLength() == 0 && x.getLP() != "_" {
                st += &format!(" [{}-1:0]",x.getLP());
            }
            else if x.getLength() != 0 {
                st += &format!(" [{}:0]",x.getLength()-1);
            }
            else {
                st += "";
            }

            if n > num {
                st += ",\n";
            }
            else {
                st += "\n";
            }
        }
        st += ");\n";
        st
}

/// LocalParameter + Wire + Reg出力関数
fn PrintLocal(PWR: Vec<wrVar>) -> String {
	if PWR.len() == 0 {
		return String::new();
	}
    let mut st = String::new();
    st += "    // ----Generate Local Parts----\n\n";
    let tmp = PWR;
    for x in tmp {
        let port_set = x.getIO();
        match port_set {
            io_p::param_ => {
                st += &format!("    localparam {} = {};\n",x.getName(), x.getValue());
            }
            io_p::none => {
                if x.getReg() {
                    st += "    reg ";
                }
                else {
                    st += "    wire ";
                }
                if x.getWidth() == 0 && x.getWP() != "_".to_string() {
                    st += &format!("[{}-1:0] ",x.getWP());
                }
                else if x.getWidth() > 1 {
                    st += &format!("[{}:0] ",x.getWidth()-1);
                }
                else {
                    st += " ";
                }

                st += &format!("{}",x.getName());

                if x.getLength() == 0 && x.getLP() != "_".to_string() {
                    st += &format!(" [{}-1:0]",x.getLP());
                }
                else if x.getLength() != 0 {
                    st += &format!(" [{}:0]",x.getLength()-1);
                }
                else {
                    st += "";
                }
                st += ";\n";
            }
            _ => return st
        }
    }
    st
}

/// assign出力関数
fn PrintAssign(Assign: Vec<Assign>) -> String {
	if Assign.len() == 0 {
		return String::new();
	}
    let mut st = String::new();
    st += "\n    // ----Generate Assign Compornent----\n\n";
    let tmp = Assign;
    for mut x in tmp {
        let LO = x.LOut();
        st += "    assign ";
        st += &DecompAST(false, LO, "", 0);
        st += " = ";
        let port_set = x.ROut();
        st += &DecompAST(false, port_set, "", 0);
        st += ";\n";
    }
    st += "\n";
    st
}

/// always出力関数
fn PrintAlways(Always: Vec<Always>) -> String {
	if Always.len() == 0 {
		return String::new();
	}
    let mut st = String::new();
    st += "\n    // ----Generate Always Block----\n\n";
    let tmp = Always.clone();
    for x in tmp {
        st += "    always@(";
        let mut n = x.P_edge.clone();
        let mut tmp_num = 1;
        let mut len = n.len();
        for y in n{
            st += &format!("posedge {}",y.getName());
            if len > tmp_num {
                st += " or ";
            }
            tmp_num += 1;
        }

        n = x.N_edge.clone();
        len = n.len();
        if tmp_num > 1 && len > 0 {st += " or "}
        tmp_num = 1;
        for y in n {
            st += &format!("negedge {}",y.getName());
            if len > tmp_num {
                st += " or ";
            }
            tmp_num += 1;
        }
        st += ") begin\n";
        for s in x.stmt.clone() {
            st += &DecompAST(false, s,&x.clone().blockout(), 2);
        }
        
        st += "    end\n";
    }
    return st;
}

/// function出力関数
fn PrintFunction(Function: Vec<Func_AST>) -> String {
	if Function.len() == 0 {
		return String::new();
	}
    let mut st = String::new();
    st += "\n    // ----Generate Function Block----\n";

    let tmp = Function.clone();
    for x in tmp {
        let e = x.top;
        if let E::Ldc(wrtop) = (*e).clone() {
            st += &format!("\n    function [{}:0] ", wrtop.getWidth()-1);
            st += &DecompAST(false, e, "", 1);
        }
		st += "(\n";
		let mut i = 0;
        for inpt in x.input.clone() {
            if let E::Ldc(wr) = (*inpt).clone() {
                if wr.getWidth() > 0 {
                    st += &format!("        input [{}:0]", wr.getWidth()-1);
                    st += &DecompAST(false, inpt, "",2);
                }
                else {
                    st += "        input ";
                    st += &DecompAST(false, inpt, "", 2);
                }
				i += 1;
				if i != x.input.len() {
					st += ",\n";
				}
            }
        }
		st += "\n    );\n";
        for s in x.stmt {
            st += &DecompAST(false, s, "", 2);
        }
        st += "    endfunction\n\n";
    }
    return st;
}

/// if_else構文出力関数--ブロック出力関数より呼び出し
fn PrintIf(If_Stmt: Vec<IfStmt_AST>, cnfg: &str, indent: i32) -> String {
    let tmp = If_Stmt;
    let mut num = 0;
    let mut st = String::new();

	let mut nonBranch  = false;
    
    for mut x in tmp {
        let n = x.getStatement();
        if num == 0 {
            let e = *x.clone().getTerms();
            match e {
                E::Null => {
                    num = 0;
					nonBranch = true;
                }
                _ => {
                    for _ in 0..indent {
                        st += "    ";
                    }
                    st += "if(";
                    num += 1;
                    st += &DecompAST(false, x.getTerms(), "", 0);
                    st += ") begin\n";
                }
            }
        }
        else if x.getIfFlag() {
            for _ in 0..indent {
                st += "    ";
            }
            st += "else if(";
            st += &DecompAST(false, x.getTerms(), "",0);
            st += ") begin\n";
        }
        else {
            for _ in 0..indent {
                st += "    ";
            }
            st += "else begin\n";
        }

		if nonBranch {
			for y in n.clone() {
            	st += &DecompAST(false, y,cnfg, indent);
        	}
			return st
		}
        for y in n.clone() {
            st += &DecompAST(false, y,cnfg, indent + 1);
        }

        for _ in 0..indent {
            st += "    ";
        }
        st += "end\n";
    }
    return st;
}

/// Case構文出力関数--ブロック出力関数より呼び出し
fn PrintCase(case_stmt: CaseStmt_AST, cnfg: &str, indent: i32) -> String {
    let mut tmp = case_stmt;
    let ctmp = tmp.clone().Select;
    let mut st = String::new();
    for _ in 0..indent {
        st += "    ";
    }
    st += &format!("case ({})\n",tmp.getCaseV().getName());
    for x in ctmp {
        let e = x.CaseT.clone();
        let ef = x.CaseS.clone();
        let tm = *e.clone();
        for _ in 0..indent+1 {
            st += "    ";
        }
        match tm {
            E::Null => {
                st += "default ";
            },
            _ => {
                st += &DecompAST(false, e,cnfg, indent + 1);
            },
        }
        st += " :";
        let n = ef.len();
        if n > 1 {st += "begin \n";}
        for y in ef {
            if n > 1 {
                st += &DecompAST(false, y,cnfg, indent + 2);
            }
            else {
                st += &DecompAST(false, y,cnfg, 0);
            }
        }
        if n > 1 {
            for _ in 0..indent+1 {
                st += "    ";
            }
            st += "end \n";
        }
    }
    for _ in 0..indent {
        st += "    ";
    }
    st += "endcase\n";
    return st;
}

/// Fsm構文出力関数--always文を生成する
fn PrintFsm(Fsm: FsmModule) -> String {
    let mut st = String::new(); 
    let tmp = Fsm.clone();
    let CLK = tmp.clone().StateClk();
    let RST = tmp.clone().StateRst();
    let Reg = tmp.clone().StateReg();
    let p = tmp.clone().StateOut(); 
    st += &format!("    always@(posedge {} or posedge {}) begin\n", _StrOut(CLK.clone()), _StrOut(RST.clone()));
    st += &format!("        if ({} == 1) begin \n            {} <= {}; \n        end\n",_StrOut(RST.clone()), _StrOut(Reg.clone()), _StrOut(tmp.clone().FirstState()));
    st += &format!("        else begin \n            {} <= {}_Next; \n        end \n    end \n\n",_StrOut(Reg.clone()),_StrOut(Reg.clone()));
    st += &format!("    always@(posedge {}) begin\n",_StrOut(CLK.clone()));
    st += &format!("        if ({}) {}_Next <= {};\n", _StrOut(RST.clone()), _StrOut(Reg.clone()), _StrOut(tmp.clone().FirstState()));
    st += "        else begin\n";
    st += &format!("            case({})\n",_StrOut(Reg.clone()));
    for s in p {
        st += &PrintState(s.clone());
    }
    st += "            endcase \n        end\n    end\n\n";

    return st;
}

/// 1Stateモデル出力関数
fn PrintState(STMT: StateModule) -> String {
    let mut s = STMT;
    let stname = s.getStateName();
    let tmp = s.getBranch();

    let mut st = String::new();

    st += &format!("                {} : begin\n",stname);
    st += &PrintIf(tmp.clone(), "Non", 5);
    st += "                end\n";

    return st;
}

/// AXIインタフェース出力関数
fn PrintAXI(AXI_Sugar: AXI, num: i32) -> String {
    let tmp = AXI_Sugar.clone();
	let mut st = String::new();
    match tmp {
        AXI::Lite(x) => { st += &PrintAXISL(x, num);}
        AXI::Slave(x) => {st += &PrintAXIS(x);}
        AXI::Master(_) => {unimplemented!();}
        AXI::Stream(_) => {unimplemented!();}
    }
    return st;
}

/// AXISLite構文出力関数--ほぼテンプレ
fn PrintAXISL(AXISL: AXISLite, count: i32) -> String {
	let tmp = AXISL.clone();
    let mut st = String::new();

    // register
	let reg_tmp = tmp.reg_array.clone();

    // address space
    let mut addr_width = 0;

    // address width
    let reg_length = tmp.reg_array.len() as i32;
    let mut reg_addr_width: i32 = 1;
    loop {
        if 2i32.pow(reg_addr_width as u32) >= (reg_length * 4 - 1) {
            break;
        }
        reg_addr_width += 1;
    }

	st += &format!("    // AXI Lite Slave Port : Number {}\n", count);
    st += &format!("    reg r_en{};\n", count);
    st += &format!("    wire w_wdata_en{};\n", count);
    st += &format!("    wire w_rdata_en{};\n\n", count);

    st += "    // wready - waddress generating\n";
    st += &format!("    always @( posedge {} ) begin\n", _StrOut(tmp.clone().clk));
    st += &format!("        if( {} ) begin\n", _StrOut(tmp.clone().rst));
    st += &format!("            r_wready{} <= 1'b0;\n            r_awready{0} <= 1'b0;\n            r_en{0} <= 1'b1;\n            r_awaddr{0} <= 0;\n",count);
    st += &format!("        end else begin\n");
    st += &format!("            if( ~r_wready{} && w_awvalid{0} && w_wvalid{0} && r_en{0} ) begin\n", count);
    st += &format!("                r_wready{0} <= 1'b1;\n            end else begin\n                r_wready{0} <= 1'b0;\n            end\n\n",count);
    st += &format!("            if( ~r_awready{} && w_awvalid{0} && w_wvalid{0} && r_en{0} ) begin\n", count);
    st += &format!("                r_awready{0} <= 1'b1;\n                r_en{0} <= 1'b0;\n                r_awaddr{0} <= i_s_awaddr{0};\n", count);
    st += &format!("            end else begin\n");
    st += &format!("                if( w_bready{} && r_bvalid{0} ) begin\n", count);
    st += &format!("                    r_en{} <= 1'b1;\n                end\n", count);
    st += &format!("                r_awready{0} <= 1'b0;\n", count);
    st += &format!("            end\n        end\n    end\n\n");

    st += &format!("    assign w_wdata_en{} = r_awready{0} && r_wready{0} && w_awvalid{0} && w_wvalid{0};\n\n", count);
    
    st += "    // wdata generating\n";
    st += &format!("    always@( posedge {} ) begin\n", _StrOut(tmp.clone().clk));
    st += &format!("        if( {} ) begin\n", _StrOut(tmp.clone().rst));

	for x in tmp.reg_array.clone() {
        st += &format!("            {} <= 32'd0;\n", _StrOut(x));
	}
    st += &format!("        end\n        else begin\n            if( w_wdata_en{} == 1'd1 ) begin\n", count);
    st += &format!("                case ( r_awaddr{}[{}:2] )\n", count, reg_addr_width-1);
    
    st += "    // generate write register\n";
    for x in reg_tmp.clone() {
        // Unpack
        let reg = x;
        st += &format!("                    {}'h{:02X} : begin\n", reg_addr_width-2, addr_width);
        for addr_count in 0..4 {
            st += &format!("                        if ( r_wstrb{}[{}] == 1'b1 ) {} <= w_wdata{0}[{}:{}];\n",
			    count, addr_count, _StrOut(reg.clone()), 8*(addr_count+1)-1, 8*addr_count);
        }

        addr_width += 1;
        st += "                    end\n";
    }
    st += "                    default: begin\n";
	for x in reg_tmp.clone() {
        st += &format!("                        {} <= {};\n", 
            _StrOut(x.clone()), _StrOut(x.clone()));
	}
    st += "                    end\n                endcase\n            end\n";

	st += "    // Local write en\n";
	let write_tmp = tmp.wLocal_write.clone();
	let mut i = -1;
	for x in write_tmp.clone() {
		i += 1;
		if let E::Null = *(x.0.clone()) {continue;}
        st += &format!("\n            if( {} ) begin \n", &DecompAST(false, x.0, "", 0));
        st += &format!("                    {} <= {};\n",
            _StrOut(reg_tmp[i as usize].clone()), &DecompAST(false, x.1, "", 0));
        st += "            end\n";
	}
    st += "        end\n    end\n\n";

    st += "    // wready - waddress generating\n";
    st += &format!("    always @( posedge {} ) begin\n", _StrOut(tmp.clone().clk));
    st += &format!("        if( {} ) begin\n", _StrOut(tmp.clone().rst));
    st += &format!("            r_bvalid{} <= 1'b0;\n",count);
    st += &format!("            r_arready{} <= 1'b0;\n            r_araddr{0} <= 0;\n",count);
    st += &format!("            r_rvalid{} <= 1'b0;\n",count);
    st += "        end else begin\n";
    
    st += &format!("            if( r_awready{} && w_awvalid{0} && ~r_bvalid{0} && r_wready{0} && w_wvalid{0} ) begin\n", count);
    st += &format!("                r_bvalid{} <= 1'b1;\n            end else if( w_bready{0} && r_bvalid{0} ) begin\n                r_bvalid{0} <= 1'b0;\n            end\n\n",count);

    st += &format!("            if( ~r_arready{} && w_arvalid{0} ) begin\n", count);
    st += &format!("                r_arready{} <= 1'b1;\n                r_araddr{0} <= i_s_araddr{0};\n            end else begin\n                r_arready{0} <= 1'b0;\n            end\n", count);

    st += &format!("            if( r_arready{} && w_arvalid{0} && ~r_rvalid{0} ) begin\n", count);
    st += &format!("                r_rvalid{} <= 1'b1;\n            end else if ( r_rvalid{0} && w_rready{0} ) begin\n                r_rvalid{0} <= 1'b0;\n            end\n", count);
    st += "        end\n    end\n\n";

    st += "    // rdata generation\n";
    st += &format!("    assign w_rdata_en{} = r_arready{0} && w_arvalid{0} && ~r_rvalid{0};\n\n", count);
    st += &format!("    always @( posedge {} ) begin\n", _StrOut(tmp.clone().clk));
    st += &format!("        if( {} ) begin\n", _StrOut(tmp.clone().rst));
    st += &format!("            r_rdata{} <= 32'd0; \n        end\n", count);
    st += "        else begin\n";
    st += &format!("            if( w_rdata_en{} ) begin\n", count);
    st += &format!("                case( r_araddr{}[{}:2] )\n", count, reg_addr_width-1);

	// 配列の生成
	i = -1;
	for x in reg_tmp.clone() {
		i += 1;
        st += &format!("                    {}'h{:02X} : r_rdata{} <= {};\n", reg_addr_width-2, i, count, _StrOut(x.clone()));
	}

    st += &format!("                    default: r_rdata{} <= 32'hDEAD_DEAD;\n                endcase\n", count);
    st += "            end\n        end\n    end\n\n";

	return st;
}

fn PrintAXIS(AXI: AXIS) -> String {
	let tmp = AXI.clone();
	let mut st = String::new();
	
	// address space
	let mut addr_width: i32 = 1;
	loop {
        if 2i32.pow(addr_width as u32) >= (tmp.length) {
            break;
        }
        addr_width += 1;
	}

	st += &format!("    // AXI-full Slave Port\n\n");

	// -- not support wrap mode --
	st += "    reg            r_axi_awv_awr_flag;\n";
	st += "    reg            r_axi_arv_arr_flag;\n";
	st += "    reg    [7:0]   r_axi_awlen_count;\n";
	st += "    reg    [7:0]   r_axi_arlen_count;\n";
	st += "    reg    [1:0]   r_axi_arburst;\n";
	st += "    reg    [1:0]   r_axi_awburst;\n\n";

	if tmp.mem {
		st += &format!("    reg [31:0] axi_mem [0:{}];\n", tmp.length-1);
		st += &format!("    always @( posedge {} ) begin\n", _StrOut(tmp.clone().clk));
		st += &format!("        if ( r_axi_wready & w_axi_wvalid ) begin\n");
		st += &format!("            axi_mem[r_axi_awaddr] <= w_axi_wdata;\n");
		st += &format!("        end else if ( axis_wen ) begin\n");
		st += &format!("            axi_mem[axis_addr] <= axis_write;\n");
		st += &format!("        end\n    end\n\n");
	}
	else {
		st += "    assign axis_wen = r_axi_wready & w_axi_wvalid;\n";
		st += &format!("    assign axis_addr = (r_axi_awv_awr_flag) ? r_axi_awaddr[{0}:2] : \n                       (r_axi_arv_arr_flag) ? r_axi_araddr[{0}:2] : 0;", addr_width+1);	
	}

	st += "    // awready - awv_awr_flag generating\n";
	st += &format!("    always @( posedge {} ) begin\n", _StrOut(tmp.clone().clk));
	st += &format!("        if ( {} ) begin\n", _StrOut(tmp.clone().rst));
	st += &format!("            r_axi_awready <= 1'b0;\n            r_axi_awv_awr_flag <= 1'b0;\n");
	st += "        end else begin\n";
	st += &format!("            if (~r_axi_awready && w_axi_awvalid && ~r_axi_awv_awr_flag && ~r_axi_arv_arr_flag ) begin\n");
	st += &format!("                r_axi_awready <= 1'b1;\n            r_axi_awv_awr_flag <= 1'b1;\n");
	st += &format!("            end else if ( w_axi_wlast && r_axi_wready ) begin\n                r_axi_awv_awr_flag <= 1'b0;\n");
	st += &format!("            end else begin\n                r_axi_awready <= 1'b0;\n");
	st += "            end\n        end\n    end\n\n";

	st += "    // waddress generation\n";
	st += &format!("    always @( posedge {} ) begin\n", _StrOut(tmp.clone().clk));
	st += &format!("        if ( {} ) begin\n", _StrOut(tmp.clone().rst));
	st += &format!("            r_axi_awaddr <= 0;\n            r_axi_awlen_count <= 0;\n            r_axi_awburst <= 0;\n            r_axi_awlen <= 0;\n");
	st += "        end else begin\n";
	st += &format!("            if ( ~r_axi_awready && w_axi_awvalid && ~r_axi_awv_awr_flag ) begin\n");
	st += &format!("                r_axi_awaddr <= i_saxi_awaddr;\n                r_axi_awburst <= i_saxi_awburst;\n                r_axi_awlen <= i_saxi_awlen;\n                r_axi_awlen_count <= 0;\n");
	st += &format!("            end else if ( ( r_axi_awlen_count <= r_axi_awlen ) && r_axi_wready && w_axi_wvalid ) begin\n");
	st += &format!("                r_axi_awlen_count <= r_axi_awlen_count + 1;\n\n");
	st += &format!("                case ( r_axi_awburst )\n");
	st += &format!("                    2'b00: begin\n                        r_axi_awaddr <= r_axi_awaddr;\n                    end\n");
	st += &format!("                    2'b01: begin\n                        r_axi_awaddr[{0}:2] <= r_axi_awaddr[{0}:2] + 1;\n                        r_axi_awaddr[1:0] <= 2'b00;\n                    end\n", addr_width+1);
	st += &format!("                    default: begin\n                        r_axi_awaddr <= r_axi_awaddr[{0}:2] + 1;\n                    end\n                endcase\n", addr_width+1);
	st += "            end\n        end\n    end\n\n";

	st += "    // wready generation\n";
	st += &format!("    always @( posedge {} ) begin\n", _StrOut(tmp.clone().clk));
	st += &format!("        if ( {} ) begin\n", _StrOut(tmp.clone().rst));
	st += &format!("            r_axi_wready <= 0;\n");
	st += "        end else begin\n";
	st += &format!("            if ( ~r_axi_wready && w_axi_wvalid && r_axi_awv_awr_flag ) begin\n                r_axi_wready <= 1'b1;\n            end else begin\n                r_axi_wready <= 1'b0;\n            end\n");
	st += "        end\n    end\n\n";

	st += "    // write response generation\n";
	st += &format!("    always @( posedge {} ) begin\n", _StrOut(tmp.clone().clk));
	st += &format!("        if ( {} ) begin\n", _StrOut(tmp.clone().rst));
	st += &format!("            r_axi_bvalid <= 0;\n");
	st += "        end else begin\n";
	st += &format!("            if ( r_axi_awv_awr_flag && r_axi_wready && w_axi_wvalid && ~r_axi_bvalid && w_axi_wlast ) begin\n");
	st += &format!("                r_axi_bvalid <= 1'b1;\n");
	st += &format!("            end else begin\n");
	st += &format!("                if ( w_axi_bready && r_axi_bvalid ) begin\n                    r_axi_bvalid <= 1'b0;\n                end\n");
	st += "            end\n        end\n    end\n\n";

	st += "    // arready - arv_arr_flag generation\n";
	st += &format!("    always @( posedge {} ) begin\n", _StrOut(tmp.clone().clk));
	st += &format!("        if ( {} ) begin\n", _StrOut(tmp.clone().rst));
	st += &format!("            r_axi_arready <= 1'b0;\n            r_axi_arv_arr_flag <= 1'b0;\n");
	st += "        end else begin\n";
	st += &format!("            if ( ~r_axi_arready && w_axi_arvalid && ~r_axi_awv_awr_flag && ~r_axi_arv_arr_flag ) begin\n");
	st += &format!("                r_axi_arready <= 1'b1;\n                r_axi_arv_arr_flag <= 1'b1;\n");
	st += &format!("            end else if ( r_axi_rvalid && w_axi_rready && r_axi_arlen_count == r_axi_arlen ) begin\n                r_axi_arv_arr_flag <= 1'b0;\n");
	st += &format!("            end else begin\n                r_axi_arready <= 1'b0;\n");
	st += "            end\n        end\n    end\n\n";

	st += "    // raddress generation\n";
	st += &format!("    always @( posedge {} ) begin\n", _StrOut(tmp.clone().clk));
	st += &format!("        if ( {} ) begin\n", _StrOut(tmp.clone().rst));
	st += &format!("            r_axi_araddr <= 0;\n            r_axi_arlen_count <= 0;\n            r_axi_arburst <= 0;\n            r_axi_arlen <= 0;\n            r_axi_rlast <= 0;\n");
	st += "        end else begin\n";
	st += &format!("            if ( ~r_axi_arready && w_axi_arvalid && ~r_axi_arv_arr_flag ) begin\n");
	st += &format!("                r_axi_araddr <= i_saxi_araddr;\n                r_axi_arburst <= i_saxi_arburst;\n                r_axi_arlen <= i_saxi_arlen;\n                r_axi_arlen_count <= 0;\n                r_axi_rlast <= 0;\n");
	st += &format!("            end else if ( ( r_axi_arlen_count <= r_axi_arlen ) && r_axi_rvalid && w_axi_rready ) begin\n");
	st += &format!("                r_axi_arlen_count <= r_axi_arlen_count + 1;\n                r_axi_rlast <= 0;\n");
	st += &format!("                case ( r_axi_arburst )\n");
	st += &format!("                    2'b00: begin\n                        r_axi_araddr <= r_axi_araddr;\n                    end\n");
	st += &format!("                    2'b01: begin\n                        r_axi_araddr[{0}:2] <= r_axi_araddr[{0}:2] + 1;\n                        r_axi_araddr[1:0] <= 2'b00;\n                    end\n", addr_width+1);
	st += &format!("                    default: begin\n                        r_axi_araddr <= r_axi_araddr[{0}:2];\n                    end\n                endcase\n", addr_width+1);
	st += &format!("            end else if ( ( r_axi_arlen_count == r_axi_arlen ) && ~r_axi_rlast && r_axi_arv_arr_flag ) begin\n                r_axi_rlast <= 1'b1;\n");
	st += &format!("            end else if ( w_axi_rready ) begin\n                r_axi_rlast <= 1'b0;\n");
	st += "            end\n        end\n    end\n\n";

	st += "    // rvalid generation\n";
	st += &format!("    always @( posedge {} ) begin\n", _StrOut(tmp.clone().clk));
	st += &format!("        if ( {} ) begin\n", _StrOut(tmp.clone().rst));
	st += &format!("            r_axi_rvalid <= 0;\n");
	st += "        end else begin\n";
	st += &format!("            if ( ~r_axi_wready && w_axi_wvalid && r_axi_awv_awr_flag ) begin\n                r_axi_rvalid <= 1'b1;\n            end else begin\n                r_axi_rvalid <= 1'b0;\n            end\n");
	st += "        end\n    end\n\n";

	st += "    assign w_axi_wdata[0+:8] = i_saxi_wstrb[0] ? i_saxi_wdata[0+:8] : 0;\n";
	st += "    assign w_axi_wdata[8+:8] = i_saxi_wstrb[1] ? i_saxi_wdata[8+:8] : 0;\n";
	st += "    assign w_axi_wdata[16+:8] = i_saxi_wstrb[2] ? i_saxi_wdata[16+:8] : 0;\n";
	st += "    assign w_axi_wdata[24+:8] = i_saxi_wstrb[3] ? i_saxi_wdata[24+:8] : 0;\n";
	

	if tmp.mem {
		st += "\n";
		st += &format!("    always @( posedge {} ) begin\n", _StrOut(tmp.clone().clk));
		st += &format!("        r_axi_rdata <= axi_mem[r_axi_araddr[{}:2]];\n", addr_width+1);
		st += &format!("        axis_read <= axi_mem[axis_addr];\n");
		st += &format!("    end\n\n");
	}
	else {
		st += "\n";
		st += &format!("    always @(*) begin\n");
		if let E::Null = *(tmp.clone().rdata) {
			st += &format!("        r_axi_rdata <= axis_read;\n");
		}
		else {
			st += &format!("        r_axi_rdata <= {};\n",  _StrOut(tmp.clone().rdata));
		}
		st += &format!("    end\n\n");
		st += &format!("    assign axis_write = w_axi_wdata;\n");
	}

	return st;
}

/// NONAST
#[macro_export]
macro_rules! Blank {
    () => (Box::new(E::Null))
}


/// FSM生成関数
pub fn Clock_Reset<T: Into<Box<E>>, U: Into<Box<E>>>(in_clk: T, in_rst: U) -> FsmModule {
    let p = wrVar::new().Reg("state", 32);
    FsmModule{clk: in_clk.into(), rst: in_rst.into(), fsm: p, State: Vec::new(), Current_state: 0}
}

/// FSMモジュール
#[derive(Debug,Clone)]
pub struct FsmModule {
    clk: Box<E>,
    rst: Box<E>,
    fsm: Box<E>,
    State: Vec<StateModule>,
    Current_state: i32,
}

impl FsmModule {
    fn FirstState(&mut self) -> Box<E> {
        self.State[0].getState()
    }
    // ステートレジスタの変更
    pub fn State(&mut self, set_state: &str) -> FsmModule {
        self.fsm = wrVar::new().Reg(set_state, 32);
        self.clone()
    }

    // ステートの追加
    pub fn AddState(&mut self, State_name: &str) -> FsmModule{
        let mut p = wrVar::new();
        self.Current_state = self.State.len() as i32;
        p.Parameter(State_name, self.Current_state);
        let tmp = StateModule{State: Box::new(E::Ldc(p)), Branch: Vec::new()};
        self.State.push(tmp);

        self.clone()
    }

    // カレントの移動
    pub fn Current(&mut self, State_name: &str) -> FsmModule {
        let mut count = 0;
        for x in &mut self.State {
            let Nx = x.getStateName();
            count+=1;
            if Nx == State_name.to_string() {
                self.Current_state = count;
            }
        }

        self.clone()
    }

    // カレントステートから次のステートへの定義
    pub fn goto<T: Into<Box<E>>>(&mut self, State_name: &str, Branch: T) -> FsmModule {
        
        let SelfS = self.fsm.clone();
        let mut st = "".to_string();
        if let E::Ldc(wr) = *SelfS.clone() { st = wr.getName().clone() };
        st = st + "_Next";
        let NState = wrVar::new().Reg(&st,0);
        let Goto_ = wrVar::new().Parameter(State_name,0);
        self.State[(self.Current_state as usize)].SetBranch(Branch.into(), F!(NState = Goto_));

        self.clone()
    }

    // 指定ステートからカレントステートへの定義(指定ステートの作成後以降に使用可)
    pub fn from<T: Into<Box<E>>>(&mut self, State_name: &str, Branch: T) -> FsmModule {
        let SelfS = self.fsm.clone();
        let mut st = "".to_string();
        if let E::Ldc(wr) = *SelfS.clone() { st = wr.getName().clone() };
        st = st + "_Next";
        let NState = wrVar::new().Reg(&st,0);
        let NameCurrentState = self.State[((self.Current_state-1) as usize)].getStateName();
        let branch = Branch.into();
        for x in &mut self.State {
            let Nx = x.getStateName();
            if Nx == State_name.to_string() {
                let Goto_ = wrVar::new().Parameter(&NameCurrentState,0);
                x.SetBranch(branch.clone(), F!(NState = Goto_));
            }
        }

        self.clone()
    }

    // セットパラメータの取得
    pub fn Param(&mut self, name: &str) -> Box<E> {
        let SelfS = self.State.clone();
        for mut x in SelfS {
            let Nx = x.getStateName();
            if Nx == name.to_string() {
                return x.getState();
            }
        }
        return Box::new(E::Null);
    }

    // 内部メソッド(ステート格納レジスタを外部に出力)
    fn StateReg(self) -> Box<E> {
        let tmp = self.clone();
        tmp.fsm
    }

    // 内部メソッド(クロックを外部に出力)
    fn StateClk(self) -> Box<E> {
        let tmp = self.clone();
        tmp.clk
    }

    // 内部メソッド(リセットを外部に出力)
    fn StateRst(self) -> Box<E> {
        let tmp = self.clone();
        tmp.rst
    }

    fn StateOut(self) -> Vec<StateModule>
    {
        let tmp = self.clone();
        tmp.State
    }
}

/// 1ステートモデル
#[derive(Debug,Clone)]
struct StateModule {
    State: Box<E>,
    Branch: Vec<IfStmt_AST>,
}

impl StateModule {
    // ステート設定
    fn SetState(&mut self, stmt: Box<E>){
        self.State = stmt
    }

    // ステート分岐先設定
    fn SetBranch<T: Into<Box<E>>, U: Into<Box<E>>>(&mut self, Terms: T, Form: U) -> bool {
        let e = *(Terms.into());
        let mut tmp = Vec::new();
        tmp.push(Form.into());
        
        match e {
            E::Null => self.Branch.push(IfStmt_AST{If_: true, IfE: Box::new(e), ST: tmp}),
            _ => self.Branch.push(IfStmt_AST{If_: true, IfE: Box::new(e), ST: tmp}),
        }
        return true;
	}
	
    fn getState(&mut self) -> Box<E> {
        let tmp = self.clone();
        tmp.State
    }

    fn getStateName(&mut self) -> String {
        let tmp = *(self.clone().State);
        match tmp {
            E::Ldc(b) => b.getName(),
            _ => "Nothing".to_string(),
        }
    }

    fn getBranch(&mut self) -> Vec<IfStmt_AST> {
        self.clone().Branch
    }
}

/// AXI wrapping enum
#[derive(Debug,Clone)]
enum AXI {
    Lite(AXISLite),
    Slave(AXIS),
    Master(AXIM),
    Stream(AXIST),
}

/// AXI Stream インタフェースの作成 - 未実装
#[derive(Debug,Clone)]
pub struct AXIST;

/// AXI Master インタフェースの作成 - 未実装
#[derive(Debug,Clone)]
pub struct AXIM;

/// AXI Slave インタフェースの作成 - 作成中
#[derive(Debug,Clone)]
pub struct AXIS {
	clk: Box<E>,
	rst: Box<E>,
	length: i32,
	mem: bool,
	rdata: Box<E>,
}

/// AXI Slave Lite インタフェースの作成
#[derive(Debug,Clone)]
pub struct AXISLite {
	clk: Box<E>,
	rst: Box<E>,
	reg_array: Vec<Box<E>>,
    wLocal_write: Vec<(Box<E>, Box<E>)>,
	current_reg: i32,
}

/// AXI Slave Lite インターフェース生成
pub fn AXIS_Lite_new<T: Into<Box<E>>, U: Into<Box<E>>>(clock: T, reset: U) -> AXISLite {
	AXISLite{
		clk: clock.into(),
		rst: reset.into(),
		reg_array: Vec::new(),
		wLocal_write: Vec::new(),
		current_reg: 0
	}
}

pub fn AXIS_new<T: Into<Box<E>>, U: Into<Box<E>>>(clock: T, reset: U) -> AXIS {
	AXIS{
		clk: clock.into(),
		rst: reset.into(),
		length: 0,
		mem: false,
		rdata: Box::new(E::Null),
	}
}

/// AXI IFのレジスタ設定トレイト
#[allow(non_camel_case_types)]
pub trait AXI_S_IF_Set<T> {
	// 数だけ指定してレジスタを生成
	fn OrderRegSet(&mut self, num: i32) -> T;
}

/// ローカルからのレジスタ制御設定トレイト
#[allow(non_camel_case_types)]
pub trait AXI_S_IF_LocalWrite<T, U>
where
	T: Into<Box<E>>,
    U: Into<Box<E>>,
{
    fn RegWrite(&mut self, write_en: U, write_data: T);
}

impl AXI_S_IF_Set<AXISLite> for AXISLite {
	fn OrderRegSet(&mut self, num: i32) -> AXISLite {
		for x in 0..num {
			let Regname = format!("{}{}", "slv_reg".to_string(), x.to_string());
			let reg = wrVar::new().Reg(&Regname, 32);
			self.reg_array.push(reg);
			self.wLocal_write.push((Box::new(E::Null), Box::new(E::Null)));
		}
		self.current_reg = num-1;
		self.clone()
	}
}

impl AXISLite {
	pub fn NamedRegSet(&mut self, name: &str) -> AXISLite {
		let reg = wrVar::new().Reg(name, 32);
		self.reg_array.push(reg);
		self.wLocal_write.push((Box::new(E::Null), Box::new(E::Null)));
		self.current_reg = self.reg_array.len() as i32 - 1;
		self.clone()
	}

	pub fn NamedReg(&mut self, name: &str) -> Box<E> {
		let SelfReg = self.reg_array.clone();
		for x in SelfReg {
			let Nx = *x.clone();
			if let E::Ldc(i) = Nx {
				if i.getName() == name.to_string() {
					return x
				}
			}
		}
		return Box::new(E::Null)
	}

	pub fn OrderReg(&mut self, num: i32) -> Box<E> {
		let SelfReg = self.reg_array.clone();
		return SelfReg[num as usize].clone();
	}
}

impl AXI_S_IF_Set<AXIS> for AXIS {
	fn OrderRegSet(&mut self, num: i32) -> AXIS {
		self.length = num;
		self.clone()
	}
}

#[allow(non_camel_case_types)]
pub trait AXIS_RegControl {
	fn write(&mut self) -> Box<E>;

	fn addr(&mut self) -> Box<E>;

	fn wen(&mut self) -> Box<E>;

	fn mem_if(&mut self) -> (Box<E>, Box<E>, Box<E>, Box<E>);
} 


// AXI4full ジェネレータを作成
#[allow(non_camel_case_types)]
impl AXIS_RegControl for AXIS {
	fn write(&mut self) -> Box<E> {
		wrVar::new().Wire("axis_write", 32)
	}

	fn addr(&mut self) -> Box<E> {
		wrVar::new().Wire("axis_addr", 32)
	}

	fn wen(&mut self) -> Box<E> {
		wrVar::new().Wire("axis_wen", 1)
	}

	fn mem_if(&mut self) -> (Box<E>, Box<E>, Box<E>, Box<E>) {
		self.mem = true;
		(wrVar::new().Wire("axis_read", 32), wrVar::new().Wire("axis_write", 32), wrVar::new().Wire("axis_wen", 1), wrVar::new().Wire("axis_addr", 32))
	}
}

#[allow(non_camel_case_types)]
pub trait AXIS_readcontrol<T>
where
	T:Into<Box<E>>,
{
	fn read(&mut self, rdata: T) -> AXIS;
}

#[allow(non_camel_case_types)]
impl<T> AXIS_readcontrol<T> for AXIS
where
	T: Into<Box<E>>,
{
	fn read(&mut self, rdata: T) -> AXIS {
		self.rdata = rdata.into();
		self.clone()
	}
}

/// AXIS Lite ローカル側データ書き込み処理設定
impl<T, U> AXI_S_IF_LocalWrite<T, U> for AXISLite
where
    T: Into<Box<E>>,
    U: Into<Box<E>>,
{
    fn RegWrite(&mut self, write_en: U, write_data: T) {
		// localwrite AXI Register
		self.wLocal_write[self.current_reg.clone() as usize] = (write_en.into(), write_data.into());
		return;
	}
}


// 基本Box<E>の分解に使用

/// AST分解メソッド
pub fn _Decomp<T: Into<Box<E>>>(e: T, Sel: &str) -> Box<E> {
    let m = *e.into();
    match m {
        E::Bin(_, ref L, ref R) => {
            if Sel == "L" {Box::new(*L.clone())}
            else if Sel == "R" {Box::new(*R.clone())}
            else {Box::new(E::Null)}
        },
        E::PL(ref D, ref T, ref F) => {
            if Sel == "D" {Box::new(*D.clone())}
            else if Sel == "T" {Box::new(*T.clone())}
            else if Sel == "F" {Box::new(*F.clone())}
            else {Box::new(E::Null)}
        },
        E::SB(ref L, ref R) => {
            if Sel == "L" {Box::new(*L.clone())}
            else if Sel == "R" {Box::new(*R.clone())}
            else {Box::new(E::Null)}
        }
        _ => Box::new(E::Null),
    }
}

/// AST文字列抽出メソッド
pub fn _StrOut<T: Into<Box<E>>>(e: T) -> String {
    let m = *e.into();
    match m {
        E::Ldc(WR) => WR.getName(),
        E::Bin(ref Op, _, _) => Op.clone(),
        _ => "Null".to_string(),
    }
}

/// AST数値抽出メソッド
pub fn _NumOut<T: Into<Box<E>>>(e: T) -> i32 {
    let m = *e.into();
    match m {
        E::Ldc(WR) => WR.getWidth(),
        E::Num(i) => i,
        _ => 0,
    }
}
