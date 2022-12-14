%define name link-grammar
%define version 5.10.4
%define release 1

Summary: A Natural Language Parser based on Link Grammar Theory

Name: %{name}
Version: %{version}
Release: %{release}
Group: System Environment/Libraries
License: LGPL

Source: http://www.abisource.com/downloads/link-grammar/%{version}/link-grammar-%{version}.tar.gz
Buildroot: /var/tmp/%{name}-%{version}-%{release}-root
URL: http://abisource.com/projects/link-grammar/

#Requires: 
BuildRequires: hunspell-devel, java-devel, jpackage-utils, libedit-devel, ant, minisat-devel

%description
The Link Grammar Parser is a syntactic parser of English, Russian,
Arabic, Persian and other languages.  It is based on Link Grammar, an theory of 
natural language syntax. Given a sentence, the system assigns to it
a syntactic structure, which consists of a set of labeled links connecting
pairs of words.  The parser also produces a HPSG-style representation of a
sentence (showing noun phrases, verb phrases, etc.).

%package devel
Summary: Support files necessary to compile applications with link-grammar.
Group: Development/Libraries
Requires: link-grammar

%description devel
Libraries, headers, and support files necessary to compile applications using link-grammar.

%prep

%setup

%build
# help configure find jni.h
export JAVA_HOME=%{java_home}

%ifarch alpha
  MYARCH_FLAGS="--host=alpha-redhat-linux"
%endif

if [ ! -f configure ]; then
CFLAGS="$RPM_OPT_FLAGS" ./autogen.sh --prefix=%{_prefix} --no-configure
fi

%configure
CFLAGS="$RPM_OPT_FLAGS" ./configure --prefix=%{_prefix} --enable-python-bindings --disable-aspell

if [ "$SMP" != "" ]; then
  (%__make "MAKE=%__make -k -j $SMP"; exit 0)
  %__make
else
%__make
fi

%install
if [ -d $RPM_BUILD_ROOT ]; then rm -r $RPM_BUILD_ROOT; fi
%__make DESTDIR=$RPM_BUILD_ROOT install
find $RPM_BUILD_ROOT/%{_libdir} -name \*.la -exec rm -f \{\} \;

%files
%defattr(644,root,root,755)
%doc LICENSE README ChangeLog
%attr(755,root,root)%{_bindir}/*
%{_libdir}/lib*.so*
%{_datadir}/link-grammar/*

%files devel
%defattr(644,root,root,755)
%{_libdir}/*.a
%{_libdir}/pkgconfig/link-grammar.pc
%{_includedir}/link-grammar/*

%clean
%__rm -r $RPM_BUILD_ROOT

%changelog
* Fri Sep 23 2016 Amir Plivatsky <amirpli@gmail.com>
- Add minisat dependency.
- Enable the SAT-solver (enabled by default).
* Fri Jun 27 2014 Linas Vepstas <linasvepstas@gmail.com>
- Freshen with newer URL, dependencies.
* Sat Feb 5 2005 Dom Lachowicz <cinamod@hotmail.com>
- Initial version
